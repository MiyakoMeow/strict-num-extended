//! Configuration structure and parsing module
//!
//! Defines configuration structures for procedural macros and `TokenStream` parsing logic.

use std::collections::HashMap;

use proc_macro2::{Ident, Span};
use syn::{Expr, Lit, parse::Parse, parse::ParseStream};

// ============================================================================
// Sign and bound type definitions for arithmetic operations
// ============================================================================

/// Sign property of a constraint type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// Non-negative values (>= 0)
    Positive,
    /// Non-positive values (<= 0)
    Negative,
    /// Any sign (including zero)
    Any,
}

/// Bound information for a constraint type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Lower bound (None means -∞)
    pub lower: Option<f64>,
    /// Upper bound (None means +∞)
    pub upper: Option<f64>,
}

impl Bounds {
    /// Check if this type is bounded (has both upper and lower bounds)
    pub const fn is_bounded(&self) -> bool {
        self.lower.is_some() && self.upper.is_some()
    }
}

/// Arithmetic operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Result of arithmetic operation type inference.
#[derive(Debug, Clone)]
pub struct ArithmeticResult {
    /// Output constraint type name
    pub output_type: Ident,
    /// Whether this operation is safe (no overflow/divide-by-zero possible)
    pub is_safe: bool,
}

// ============================================================================
// Configuration structure definitions
// ============================================================================

/// Main configuration structure.
pub struct TypeConfig {
    /// List of constraint definitions.
    pub constraints: Vec<ConstraintDef>,
    /// List of constraint type definitions.
    pub constraint_types: Vec<TypeDef>,
    /// Arithmetic operation results: (op, `lhs_name`, `rhs_name`) -> `ArithmeticResult`
    pub arithmetic_results: HashMap<(ArithmeticOp, String, String), ArithmeticResult>,
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
    /// Sign property of this constraint.
    pub sign: Sign,
    /// Bound information of this constraint.
    pub bounds: Bounds,
    /// Whether this constraint excludes zero.
    pub excludes_zero: bool,
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

            // Parse sign and bounds from conditions
            let (sign, bounds, excludes_zero) = parse_type_properties(&conditions);

            // Generate constraint definition
            let doc = generate_auto_doc(&type_name, &conditions);
            constraints.push(ConstraintDef {
                name: type_name.clone(),
                doc,
                validate: validate_expr.clone(),
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

        Ok(TypeConfig {
            constraints,
            constraint_types,
            arithmetic_results,
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
    s.parse().unwrap_or(0.0)
}

// ============================================================================
// Arithmetic operation inference
// ============================================================================

/// Compute arithmetic results for all constraint combinations.
fn compute_all_arithmetic_results(
    constraints: &[ConstraintDef],
) -> HashMap<(ArithmeticOp, String, String), ArithmeticResult> {
    let mut results = HashMap::new();
    let ops = [
        ArithmeticOp::Add,
        ArithmeticOp::Sub,
        ArithmeticOp::Mul,
        ArithmeticOp::Div,
    ];

    for lhs in constraints {
        for rhs in constraints {
            for &op in &ops {
                let result = compute_arithmetic_result(op, lhs, rhs, constraints);
                results.insert((op, lhs.name.to_string(), rhs.name.to_string()), result);
            }
        }
    }

    results
}

/// Compute the result type and safety for a single arithmetic operation.
fn compute_arithmetic_result(
    op: ArithmeticOp,
    lhs: &ConstraintDef,
    rhs: &ConstraintDef,
    all_constraints: &[ConstraintDef],
) -> ArithmeticResult {
    let (output_sign, output_excludes_zero, is_safe) = match op {
        ArithmeticOp::Add => compute_add_properties(lhs, rhs),
        ArithmeticOp::Sub => compute_sub_properties(lhs, rhs),
        ArithmeticOp::Mul => compute_mul_properties(lhs, rhs),
        ArithmeticOp::Div => compute_div_properties(lhs, rhs),
    };

    // Find the best matching constraint type for the output
    let output_type = find_matching_constraint(output_sign, output_excludes_zero, all_constraints);

    ArithmeticResult {
        output_type,
        is_safe,
    }
}

/// Compute output properties for addition.
fn compute_add_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Addition can never be safe (overflow possible)
    let is_safe = false;

    // Sign rules for addition:
    // Positive + Positive = Positive
    // Negative + Negative = Negative
    // Mixed = Any
    let output_sign = match (lhs.sign, rhs.sign) {
        (Sign::Positive, Sign::Positive) => Sign::Positive,
        (Sign::Negative, Sign::Negative) => Sign::Negative,
        _ => Sign::Any,
    };

    // NonZero + NonZero with same sign preserves NonZero
    let output_excludes_zero =
        lhs.excludes_zero && rhs.excludes_zero && lhs.sign == rhs.sign && lhs.sign != Sign::Any;

    (output_sign, output_excludes_zero, is_safe)
}

/// Compute output properties for subtraction.
fn compute_sub_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Subtraction can never be safe (overflow possible)
    let is_safe = false;

    // a - b: negate rhs sign
    let rhs_negated_sign = match rhs.sign {
        Sign::Positive => Sign::Negative,
        Sign::Negative => Sign::Positive,
        Sign::Any => Sign::Any,
    };

    // Sign rules (same as add with negated rhs):
    // Positive - Negative = Positive + Positive = Positive
    // Negative - Positive = Negative + Negative = Negative
    // Others = Any
    let output_sign = match (lhs.sign, rhs_negated_sign) {
        (Sign::Positive, Sign::Positive) => Sign::Positive,
        (Sign::Negative, Sign::Negative) => Sign::Negative,
        _ => Sign::Any,
    };

    // NonZero - NonZero with opposite signs (after negation, same sign) preserves NonZero
    let output_excludes_zero = lhs.excludes_zero
        && rhs.excludes_zero
        && lhs.sign == rhs_negated_sign
        && lhs.sign != Sign::Any;

    (output_sign, output_excludes_zero, is_safe)
}

/// Compute output properties for multiplication.
const fn compute_mul_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Multiplication is safe if both operands are bounded (result stays bounded)
    let is_safe = lhs.bounds.is_bounded() && rhs.bounds.is_bounded();

    // Sign rules for multiplication:
    // Positive × Positive = Positive
    // Negative × Negative = Positive
    // Positive × Negative = Negative
    // Any × Any = Any
    let output_sign = match (lhs.sign, rhs.sign) {
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
        _ => Sign::Any,
    };

    // NonZero × NonZero = NonZero
    let output_excludes_zero = lhs.excludes_zero && rhs.excludes_zero;

    (output_sign, output_excludes_zero, is_safe)
}

/// Compute output properties for division.
const fn compute_div_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Division is never safe (divide by zero possible, or result overflow)
    let is_safe = false;

    // Sign rules for division (same as multiplication):
    let output_sign = match (lhs.sign, rhs.sign) {
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => Sign::Positive,
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => Sign::Negative,
        _ => Sign::Any,
    };

    // NonZero ÷ NonZero = NonZero (if lhs is nonzero)
    let output_excludes_zero = lhs.excludes_zero;

    (output_sign, output_excludes_zero, is_safe)
}

/// Find the best matching constraint type for given output properties.
fn find_matching_constraint(
    sign: Sign,
    excludes_zero: bool,
    constraints: &[ConstraintDef],
) -> Ident {
    // Priority: more specific types first
    // 1. Try to find exact match (sign + excludes_zero)
    // 2. Fall back to sign-only match
    // 3. Fall back to Fin

    for c in constraints {
        if c.sign == sign
            && c.excludes_zero == excludes_zero
            && c.bounds.lower.is_none()
            && c.bounds.upper.is_none()
        {
            return c.name.clone();
        }
    }

    // Relaxed match: just sign and excludes_zero
    for c in constraints {
        if c.sign == sign && c.excludes_zero == excludes_zero {
            // Skip bounded types for unbounded results
            if c.bounds.is_bounded() {
                continue;
            }
            return c.name.clone();
        }
    }

    // Further relaxed: just sign
    for c in constraints {
        if c.sign == sign && !c.bounds.is_bounded() {
            return c.name.clone();
        }
    }

    // Default to Fin
    Ident::new("Fin", Span::call_site())
}
