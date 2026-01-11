//! Arithmetic operation type inference
//!
//! Contains all logic for inferring output types and safety of arithmetic operations.

use std::collections::HashMap;

use proc_macro2::{Ident, Span};

use super::config_types::{ArithmeticOp, ArithmeticResult, Bounds, ConstraintDef, Sign};

// ============================================================================
// Arithmetic operation inference
// ============================================================================

/// Checks if all boundary operation results are finite.
///
/// Performs actual operations on the boundary values of lhs and rhs,
/// checking if all possible extreme value combinations produce finite results.
pub fn bounds_op_is_finite(lhs: &Bounds, rhs: &Bounds, op: impl Fn(f64, f64) -> f64) -> bool {
    let l_min = lhs.lower.unwrap_or(f64::MIN);
    let l_max = lhs.upper.unwrap_or(f64::MAX);
    let r_min = rhs.lower.unwrap_or(f64::MIN);
    let r_max = rhs.upper.unwrap_or(f64::MAX);

    // Compute all possible extreme value combinations
    let results: [f64; 4] = [
        op(l_min, r_min),
        op(l_min, r_max),
        op(l_max, r_min),
        op(l_max, r_max),
    ];

    results.iter().all(|r: &f64| r.is_finite())
}

/// Checks if all boundary division results are finite.
///
/// Division requires special handling: if the divisor excludes zero,
/// use the minimum positive number as the divisor's lower bound.
pub fn bounds_div_is_finite(lhs: &Bounds, rhs: &Bounds, rhs_excludes_zero: bool) -> bool {
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
pub fn compute_all_arithmetic_results(
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
        0.0f64
    };
    let upper_abs = if let Some(v) = bounds.upper {
        v.abs()
    } else {
        0.0f64
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
