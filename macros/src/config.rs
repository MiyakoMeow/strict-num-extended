//! Configuration structure and parsing module
//!
//! Defines configuration structures for procedural macros and `TokenStream` parsing logic.

use proc_macro2::{Ident, Span};
use syn::{Expr, Lit, parse::Parse, parse::ParseStream};

// ============================================================================
// Configuration structure definitions
// ============================================================================

/// Main configuration structure.
pub struct TypeConfig {
    /// List of constraint definitions.
    pub constraints: Vec<ConstraintDef>,
    /// List of constraint type definitions.
    pub constraint_types: Vec<TypeDef>,
}

/// Single constraint definition.
pub struct ConstraintDef {
    /// Constraint name.
    pub name: Ident,
    /// Constraint documentation.
    pub doc: String,
    /// Validation expression.
    pub validate: String,
    /// Name of the constraint type after negation (e.g., Positive -> Negative).
    pub neg_constraint_name: Option<Ident>,
    /// Raw conditions before adding `is_finite()` check (used for negation calculation).
    pub raw_conditions: Vec<String>,
}

/// Type definition (single constraint).
pub struct TypeDef {
    /// Type name.
    pub type_name: Ident,
    /// List of floating-point types.
    pub float_types: Vec<Ident>,
    /// Constraint name.
    pub constraint_name: Ident,
}

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

            // Automatically add is_finite check, then combine all conditions (AND logic)
            let finite_check = "value.is_finite()";
            let mut all_conditions = vec![finite_check.to_string()];
            all_conditions.extend(conditions.clone());
            let validate_expr = all_conditions.join(" && ");

            // Generate constraint definition
            let doc = generate_auto_doc(&type_name, &conditions);
            constraints.push(ConstraintDef {
                name: type_name.clone(),
                doc,
                validate: validate_expr.clone(),
                neg_constraint_name: None, // Will be calculated later
                raw_conditions: conditions,
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
        for i in 0..constraints.len() {
            let negated_conditions = negate_conditions(&constraints[i].raw_conditions);

            // Find a matching constraint in the list
            for other in &constraints {
                if conditions_match(&negated_conditions, &other.raw_conditions) {
                    constraints[i].neg_constraint_name = Some(other.name.clone());
                    break;
                }
            }
        }

        Ok(TypeConfig {
            constraints,
            constraint_types,
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

/// Automatically generate documentation
fn generate_auto_doc(type_name: &Ident, conditions: &[String]) -> String {
    let name_str = type_name.to_string();

    let base_desc = match name_str.as_str() {
        "Fin" => "Finite floating-point value",
        "Positive" => "Positive floating-point value (> 0, finite)",
        "Negative" => "Negative floating-point value (< 0, finite)",
        "NonZero" => "Non-zero floating-point value",
        "Normalized" => "Normalized floating-point value (0.0 <= value <= 1.0)",
        "NegativeNormalized" => "Negative normalized floating-point value (-1.0 <= value <= 0.0)",
        "NonZeroPositive" => "Non-zero positive floating-point value (> 0, finite)",
        "NonZeroNegative" => "Non-zero negative floating-point value (< 0, finite)",
        _ => &format!("Finite floating-point value: {}", name_str),
    };

    let conditions_str = conditions.join(" && ");
    format!("{}\n\nValidation: {}", base_desc, conditions_str)
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
