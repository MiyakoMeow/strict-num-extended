//! Configuration structure and parsing module
//!
//! Defines configuration structures for procedural macros and `TokenStream` parsing logic.

mod config_arithmetic;
mod config_types;

// Re-export all types
pub use config_arithmetic::*;
pub use config_types::*;

use proc_macro2::{Ident, Span};
use syn::{Expr, Lit, parse::Parse, parse::ParseStream};

// ============================================================================
// Parse trait implementations
// ============================================================================

impl Parse for TypeConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse bracketed content: [ ... ]
        let content;
        syn::bracketed!(content in input);

        let mut constraints = Vec::new();
        let mut constraint_types = Vec::new();

        // Parse each type definition
        while !content.is_empty() {
            // Parse (TypeName, ["condition1", "condition2", ...])
            let paren_content;
            syn::parenthesized!(paren_content in content);

            let type_name: Ident = paren_content.parse()?;
            paren_content.parse::<syn::Token![,]>()?;

            let bracket_content;
            syn::bracketed!(bracket_content in &paren_content);

            // Parse condition list
            let mut conditions = Vec::new();
            while !bracket_content.is_empty() {
                let expr: Expr = bracket_content.parse()?;
                let condition = match &expr {
                    Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }) => normalize_condition(&s.value()),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            expr,
                            "Expected string literal for validation condition",
                        ));
                    }
                };
                conditions.push(condition);

                // If not the last one, parse comma
                if !bracket_content.is_empty() {
                    bracket_content.parse::<syn::Token![,]>()?;
                }
            }

            // Parse sign and bounds from conditions
            let (sign, bounds, excludes_zero) = parse_type_properties(&conditions);

            // Generate constraint definition
            constraints.push(ConstraintDef {
                name: type_name.clone(),
                neg_constraint_name: None, // Will be calculated later
                raw_conditions: conditions,
                sign,
                bounds,
                excludes_zero,
            });

            // Generate type definition (automatically add f32 and f64)
            let type_name_clone = type_name.clone();
            constraint_types.push(TypeDef {
                type_name,
                float_types: vec![
                    Ident::new("f32", Span::call_site()),
                    Ident::new("f64", Span::call_site()),
                ],
                constraint_name: type_name_clone,
            });

            let _ = content.parse::<syn::Token![,]>();
        }

        // Calculate negation mappings for all constraints
        // Build a list of (name, raw_conditions) first to avoid borrow checker issues
        let constraints_data: Vec<_> = constraints
            .iter()
            .map(|c| (c.name.clone(), c.raw_conditions.clone()))
            .collect();

        // Now calculate negation mappings
        for constraint in &mut constraints {
            let negated_conditions = negate_conditions(&constraint.raw_conditions);

            // Find a matching constraint in the list
            for (other_name, other_raw) in &constraints_data {
                if conditions_match(&negated_conditions, other_raw) {
                    constraint.neg_constraint_name = Some(other_name.clone());
                    break;
                }
            }
        }

        // Calculate arithmetic operation results
        let arithmetic_results = compute_all_arithmetic_results(&constraints);

        // Parse second parameter: alias definitions (optional)
        let mut type_aliases = Vec::new();

        if input.parse::<syn::Token![,]>().is_ok() {
            // Has second parameter, parse alias list
            let alias_content;
            syn::bracketed!(alias_content in input);

            while !alias_content.is_empty() {
                let paren_content;
                syn::parenthesized!(paren_content in alias_content);

                let original_name: Ident = paren_content.parse()?;
                paren_content.parse::<syn::Token![,]>()?;
                let alias_name: Ident = paren_content.parse()?;

                type_aliases.push(TypeAliasDef {
                    original_name,
                    alias_name,
                });

                if !alias_content.is_empty() {
                    alias_content.parse::<syn::Token![,]>()?;
                }
            }

            // Validate aliases
            for alias_def in &type_aliases {
                // Check if original type exists
                let original_exists = constraints
                    .iter()
                    .any(|c| c.name == alias_def.original_name);

                if !original_exists {
                    return Err(syn::Error::new_spanned(
                        &alias_def.original_name,
                        format!(
                            "Alias references non-existent type '{}'",
                            alias_def.original_name
                        ),
                    ));
                }

                // Check if alias name conflicts with existing types
                let alias_conflicts = constraints.iter().any(|c| c.name == alias_def.alias_name);

                if alias_conflicts {
                    return Err(syn::Error::new_spanned(
                        &alias_def.alias_name,
                        format!(
                            "Alias name '{}' conflicts with existing type",
                            alias_def.alias_name
                        ),
                    ));
                }
            }
        }

        Ok(TypeConfig {
            constraints,
            constraint_types,
            arithmetic_results,
            type_aliases,
        })
    }
}

// ============================================================================
// Simplified format helpers
// ============================================================================

/// Normalize validation condition string by automatically adding `value` prefix
fn normalize_condition(condition: &str) -> String {
    format!("value {}", condition.trim())
}

// ============================================================================
// Negation constraint matching helpers
// ============================================================================

/// Apply negation to a list of constraint conditions.
///
/// # Rules
/// - `">= x"` becomes `"<= -x"`
/// - `"<= x"` becomes `">= -x"`
/// - `"> x"` becomes `"< -x"`
/// - `"< x"` becomes `"> -x"`
/// - `"!= x"` stays `"!= x"` (reflexive)
///
/// # Examples
/// - `">= 0.0"` → `"<= 0.0"` (since -0.0 == 0.0)
/// - `">= -1.0"` → `"<= 1.0"` (double negation)
/// - `"<= 0.0"` → `">= 0.0"` (since -0.0 == 0.0)
#[expect(clippy::option_if_let_else)] // if-let-else is clearer here
fn negate_conditions(conditions: &[String]) -> Vec<String> {
    conditions
        .iter()
        .map(|cond| {
            // Remove "value " prefix if present
            let cond = cond.strip_prefix("value ").unwrap_or(cond);

            // Apply negation rules with double negation handling
            if let Some(rest) = cond.strip_prefix(">=") {
                let rest = rest.trim();
                // Handle double negation: >= -x becomes <= x
                if let Some(num) = rest.strip_prefix('-') {
                    format!("<= {}", num)
                } else {
                    format!("<= -{}", rest)
                }
            } else if let Some(rest) = cond.strip_prefix("<=") {
                let rest = rest.trim();
                // Handle double negation: <= -x becomes >= x
                if let Some(num) = rest.strip_prefix('-') {
                    format!(">= {}", num)
                } else {
                    format!(">= -{}", rest)
                }
            } else if let Some(rest) = cond.strip_prefix(">") {
                let rest = rest.trim();
                // Handle double negation: > -x becomes < x
                if let Some(num) = rest.strip_prefix('-') {
                    format!("< {}", num)
                } else {
                    format!("< -{}", rest)
                }
            } else if let Some(rest) = cond.strip_prefix("<") {
                let rest = rest.trim();
                // Handle double negation: < -x becomes > x
                if let Some(num) = rest.strip_prefix('-') {
                    format!("> {}", num)
                } else {
                    format!("> -{}", rest)
                }
            } else if cond.starts_with("!=") {
                // Inequality stays the same
                cond.to_string()
            } else {
                // Unknown condition, keep as-is
                cond.to_string()
            }
        })
        .map(|s| normalize_condition(&s)) // Re-add "value " prefix
        .collect()
}

/// Normalize floating-point representation (handle -0.0 == 0.0).
fn normalize_float_repr(s: String) -> String {
    // Replace -0.0 with 0.0 in various contexts
    let s = s.replace("-0.0", "0.0");
    // Also handle cases with extra spaces like ">= - 0.0"
    let s = s.replace("- 0.0", "0.0");
    // Also handle "= -0.0" -> "= 0.0"

    s.replace("= -0.0", "= 0.0")
}

/// Check if two constraint condition lists match (order-insensitive).
fn conditions_match(a: &[String], b: &[String]) -> bool {
    // Normalize and compare
    let a_normalized: Vec<String> = a.iter().map(|s| normalize_float_repr(s.clone())).collect();

    let b_normalized: Vec<String> = b.iter().map(|s| normalize_float_repr(s.clone())).collect();

    // Sort and compare (ignore order)
    let mut a_sorted = a_normalized;
    let mut b_sorted = b_normalized;
    a_sorted.sort();
    b_sorted.sort();

    a_sorted == b_sorted
}

// ============================================================================
// Type property parsing helpers
// ============================================================================

/// Parse type properties (sign, bounds, `excludes_zero`) from conditions.
fn parse_type_properties(conditions: &[String]) -> (Sign, Bounds, bool) {
    let mut lower_bound: Option<f64> = None;
    let mut upper_bound: Option<f64> = None;
    let mut excludes_zero = false;
    let mut has_positive_constraint = false;
    let mut has_negative_constraint = false;

    for cond in conditions {
        let cond = cond.strip_prefix("value ").unwrap_or(cond);

        if let Some(rest) = cond.strip_prefix(">=") {
            let val = parse_float_value(rest.trim());
            lower_bound = Some(lower_bound.map_or(val, |v| v.max(val)));
            if val >= 0.0 {
                has_positive_constraint = true;
            }
        } else if let Some(rest) = cond.strip_prefix("<=") {
            let val = parse_float_value(rest.trim());
            upper_bound = Some(upper_bound.map_or(val, |v| v.min(val)));
            if val <= 0.0 {
                has_negative_constraint = true;
            }
        } else if let Some(rest) = cond.strip_prefix(">") {
            let val = parse_float_value(rest.trim());
            // > x means lower bound is x (exclusive)
            lower_bound = Some(lower_bound.map_or(val, |v| v.max(val)));
            if val >= 0.0 {
                has_positive_constraint = true;
                excludes_zero = true;
            }
        } else if let Some(rest) = cond.strip_prefix("<") {
            let val = parse_float_value(rest.trim());
            // < x means upper bound is x (exclusive)
            upper_bound = Some(upper_bound.map_or(val, |v| v.min(val)));
            if val <= 0.0 {
                has_negative_constraint = true;
                excludes_zero = true;
            }
        } else if let Some(rest) = cond.strip_prefix("!=") {
            let val = parse_float_value(rest.trim());
            if val == 0.0 {
                excludes_zero = true;
            }
        }
    }

    // Determine sign based on constraints
    let sign = if has_positive_constraint && !has_negative_constraint {
        Sign::Positive
    } else if has_negative_constraint && !has_positive_constraint {
        Sign::Negative
    } else {
        Sign::Any
    };

    let bounds = Bounds {
        lower: lower_bound,
        upper: upper_bound,
    };

    (sign, bounds, excludes_zero)
}

/// Parse a float value from a string.
fn parse_float_value(s: &str) -> f64 {
    match s {
        "PI" => core::f64::consts::PI,
        "-PI" => -core::f64::consts::PI,
        _ => s.parse().unwrap_or(0.0),
    }
}

// ============================================================================
// TypeConfig methods
// ============================================================================

impl TypeConfig {
    /// Find a constraint type by its properties (sign, bounds, `excludes_zero`).
    ///
    /// This allows looking up types by their mathematical constraints rather than
    /// by hardcoded type names, making the system more extensible.
    pub fn find_type_by_constraints(
        &self,
        sign: Sign,
        bounds: &Bounds,
        excludes_zero: bool,
    ) -> Option<Ident> {
        self.constraints
            .iter()
            .find(|c| c.sign == sign && c.bounds == *bounds && c.excludes_zero == excludes_zero)
            .map(|c| c.name.clone())
    }
}
