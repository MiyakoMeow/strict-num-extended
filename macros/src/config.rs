//! Configuration structure and parsing module
//!
//! Defines configuration structures for procedural macros and `TokenStream` parsing logic.

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;

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

/// Gets standard arithmetic operator definition array
///
/// Contains four basic arithmetic operations (addition, subtraction, multiplication, division)
/// and their corresponding:
/// - Operator enum
/// - Trait name (e.g., "Add")
/// - Method name (e.g., "add")
/// - Operator symbol (e.g., quote! { + })
///
/// # Returns
///
/// An array containing all standard arithmetic operation definitions
pub fn get_standard_arithmetic_ops() -> [(
    ArithmeticOp,
    &'static str,
    &'static str,
    proc_macro2::TokenStream,
); 4] {
    use quote::quote;

    [
        (ArithmeticOp::Add, "Add", "add", quote! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", quote! { * }),
        (ArithmeticOp::Div, "Div", "div", quote! { / }),
    ]
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

/// Checks if all boundary operation results are finite.
///
/// Performs actual operations on the boundary values of lhs and rhs,
/// checking if all possible extreme value combinations produce finite results.
fn bounds_op_is_finite(lhs: &Bounds, rhs: &Bounds, op: impl Fn(f64, f64) -> f64) -> bool {
    let l_min = lhs.lower.unwrap_or(f64::MIN);
    let l_max = lhs.upper.unwrap_or(f64::MAX);
    let r_min = rhs.lower.unwrap_or(f64::MIN);
    let r_max = rhs.upper.unwrap_or(f64::MAX);

    // Compute all possible extreme value combinations
    let results = [
        op(l_min, r_min),
        op(l_min, r_max),
        op(l_max, r_min),
        op(l_max, r_max),
    ];

    results.iter().all(|r| r.is_finite())
}

/// Checks if all boundary division results are finite.
///
/// Division requires special handling: if the divisor excludes zero,
/// use the minimum positive number as the divisor's lower bound.
fn bounds_div_is_finite(lhs: &Bounds, rhs: &Bounds, rhs_excludes_zero: bool) -> bool {
    let l_min = lhs.lower.unwrap_or(f64::MIN);
    let l_max = lhs.upper.unwrap_or(f64::MAX);

    // For divisor, if it excludes zero, avoid using 0 as the boundary value
    let r_min = if rhs_excludes_zero && rhs.lower == Some(0.0) {
        f64::MIN_POSITIVE
    } else {
        rhs.lower.unwrap_or(f64::MIN)
    };
    let r_max = if rhs_excludes_zero && rhs.upper == Some(0.0) {
        -f64::MIN_POSITIVE
    } else {
        rhs.upper.unwrap_or(f64::MAX)
    };

    // Compute all possible extreme value combinations
    let results = [l_min / r_min, l_min / r_max, l_max / r_min, l_max / r_max];

    results.iter().all(|r| r.is_finite())
}

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
    let output_type = find_matching_constraint(
        op,
        output_sign,
        output_excludes_zero,
        lhs,
        rhs,
        all_constraints,
    );

    ArithmeticResult {
        output_type,
        is_safe,
    }
}

/// Compute output properties for addition.
fn compute_add_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Safe when signs differ (Positive + Negative or Negative + Positive)
    // and all boundary operation results are finite
    let signs_differ = matches!(
        (lhs.sign, rhs.sign),
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive)
    );
    let is_safe = signs_differ && bounds_op_is_finite(&lhs.bounds, &rhs.bounds, |a, b| a + b);

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
    // Safe when signs are the same (Positive - Positive or Negative - Negative)
    // and all boundary operation results are finite
    let signs_same = lhs.sign == rhs.sign && lhs.sign != Sign::Any;
    let is_safe = signs_same && bounds_op_is_finite(&lhs.bounds, &rhs.bounds, |a, b| a - b);

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

/// Helper to compute the maximum absolute value from bounds
const fn max_abs_value(bounds: Bounds) -> f64 {
    let lower_abs = if let Some(v) = bounds.lower {
        v.abs()
    } else {
        0.0
    };
    let upper_abs = if let Some(v) = bounds.upper {
        v.abs()
    } else {
        0.0
    };
    lower_abs.max(upper_abs)
}

/// Compute multiplication result bounds based on operand signs and bounds.
fn compute_mul_result_bounds(
    lhs: &ConstraintDef,
    rhs: &ConstraintDef,
) -> (Option<f64>, Option<f64>) {
    let max_abs_lhs = max_abs_value(lhs.bounds);
    let max_abs_rhs = max_abs_value(rhs.bounds);
    let max_abs_result = max_abs_lhs * max_abs_rhs;

    match (lhs.sign, rhs.sign) {
        // Both positive or both negative → positive result [0, max]
        (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
            (Some(0.0), Some(max_abs_result))
        }
        // Different signs → negative result [-max, 0]
        (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
            (Some(-max_abs_result), Some(0.0))
        }
        // Any sign → symmetric bounds [-max, max]
        _ => (Some(-max_abs_result), Some(max_abs_result)),
    }
}

/// Filter constraints by sign and `excludes_zero` properties.
fn filter_constraints_by_properties(
    constraints: &[ConstraintDef],
    sign: Sign,
    excludes_zero: bool,
) -> Vec<&ConstraintDef> {
    constraints
        .iter()
        .filter(|c| c.sign == sign && c.excludes_zero == excludes_zero)
        .collect()
}

/// Filter constraints by sign only.
fn filter_constraints_by_sign(constraints: &[ConstraintDef], sign: Sign) -> Vec<&ConstraintDef> {
    constraints.iter().filter(|c| c.sign == sign).collect()
}

/// Compute output properties for multiplication.
fn compute_mul_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Safe when both operands are bounded and all boundary operation results are finite
    let both_bounded = lhs.bounds.is_bounded() && rhs.bounds.is_bounded();
    let is_safe = both_bounded && bounds_op_is_finite(&lhs.bounds, &rhs.bounds, |a, b| a * b);

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
fn compute_div_properties(lhs: &ConstraintDef, rhs: &ConstraintDef) -> (Sign, bool, bool) {
    // Safe when dividend is in [-1.0, 1.0], divisor is non-zero,
    // and all boundary operation results are finite
    let lhs_in_unit_range = if let (Some(lower), Some(upper)) = (lhs.bounds.lower, lhs.bounds.upper)
    {
        lower >= -1.0 && upper <= 1.0
    } else {
        false
    };
    let is_safe = lhs_in_unit_range
        && rhs.excludes_zero
        && bounds_div_is_finite(&lhs.bounds, &rhs.bounds, rhs.excludes_zero);

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
    op: ArithmeticOp,
    sign: Sign,
    excludes_zero: bool,
    lhs: &ConstraintDef,
    rhs: &ConstraintDef,
    constraints: &[ConstraintDef],
) -> Ident {
    // Check if both operands have the same bounded range
    let operands_have_same_bounds = lhs.bounds.is_bounded()
        && rhs.bounds.is_bounded()
        && lhs.bounds.lower == rhs.bounds.lower
        && lhs.bounds.upper == rhs.bounds.upper;

    // Collect all matching constraints (sign + excludes_zero)
    let matches = filter_constraints_by_properties(constraints, sign, excludes_zero);

    // If we found exact matches
    if !matches.is_empty() {
        // For multiplication, compute the actual result bounds and match them
        if matches!(op, ArithmeticOp::Mul) {
            // Check if both operands are bounded (even if bounds are different)
            if lhs.bounds.is_bounded() && rhs.bounds.is_bounded() {
                // Compute result bounds for bounded multiplication
                let (result_lower, result_upper) = compute_mul_result_bounds(lhs, rhs);

                // First, try to find a bounded type with the computed result bounds
                for c in &matches {
                    if c.bounds.is_bounded()
                        && c.bounds.lower == result_lower
                        && c.bounds.upper == result_upper
                    {
                        return c.name.clone();
                    }
                }

                // If exact bounds match not found, prefer bounded types over unbounded
                for c in &matches {
                    if c.bounds.is_bounded() {
                        return c.name.clone();
                    }
                }
            }
        }

        // Prefer unbounded types over bounded types
        for c in &matches {
            if c.bounds.lower.is_none() && c.bounds.upper.is_none() {
                return c.name.clone();
            }
        }

        // No unbounded types found, return the first bounded match
        return matches
            .first()
            .expect("matches should not be empty at this point")
            .name
            .clone();
    }

    // No exact match, try relaxed match on sign only
    let sign_matches = filter_constraints_by_sign(constraints, sign);

    if !sign_matches.is_empty() {
        // When operands have same bounds, prefer bounded types that match those bounds
        if operands_have_same_bounds {
            let operand_lower = lhs.bounds.lower;
            let operand_upper = lhs.bounds.upper;

            // Try to find a bounded type with the same bounds
            for c in &sign_matches {
                if c.bounds.is_bounded()
                    && c.bounds.lower == operand_lower
                    && c.bounds.upper == operand_upper
                {
                    return c.name.clone();
                }
            }
        }

        // Prefer unbounded types
        for c in &sign_matches {
            if c.bounds.lower.is_none() && c.bounds.upper.is_none() {
                return c.name.clone();
            }
        }

        // All are bounded, return the first one
        return sign_matches
            .first()
            .expect("sign_matches should not be empty at this point")
            .name
            .clone();
    }

    // Default to Fin
    Ident::new("Fin", Span::call_site())
}
