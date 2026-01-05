//! Arithmetic operations tests
//!
//! Tests for type-safe arithmetic operations between different constraint types.

// Strict floating-point comparisons, unwrap usage, and variable shadowing in test code are justified
#![allow(clippy::float_cmp, clippy::unwrap_used, clippy::shadow_unrelated)]

use strict_num_extended::*;

/// Unified macro for testing arithmetic operations
///
/// Supports multiple calling formats:
/// - Basic: `test_arith!(name`, `TypeA`, a, op, `TypeB`, b, `ResultType`, expected)
/// - Option LHS: `test_arith!(name`, Option<TypeA>, Some(a), op, `TypeB`, b, Option<ResultType>, Some(expected))
/// - None result: `test_arith!(name`, `TypeA`, a, op, `TypeB`, b, `ResultType`, None)
#[macro_export]
macro_rules! test_arith {
    // Option LHS with Some result
    ($test_name:ident, Option<$TypeA:ty>, Some($a:expr), $op:tt, $TypeB:ty, $b:expr, Option<$ResultType:ty>, Some($expected:expr)) => {
        #[test]
        fn $test_name() {
            let a: Option<$TypeA> = Some(<$TypeA>::new_const($a));
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = a $op B;
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), $expected);
        }
    };
    // Option LHS with None result
    ($test_name:ident, Option<$TypeA:ty>, Some($a:expr), $op:tt, $TypeB:ty, $b:expr, Option<$ResultType:ty>, None) => {
        #[test]
        fn $test_name() {
            let a: Option<$TypeA> = Some(<$TypeA>::new_const($a));
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = a $op B;
            assert!(result.is_none());
        }
    };
    // Concrete LHS with Some result (Option output)
    ($test_name:ident, $TypeA:ty, $a:expr, $op:tt, $TypeB:ty, $b:expr, Option<$ResultType:ty>, Some($expected:expr)) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = A $op B;
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), $expected);
        }
    };
    // Concrete LHS with None result
    ($test_name:ident, $TypeA:ty, $a:expr, $op:tt, $TypeB:ty, $b:expr, Option<$ResultType:ty>, None) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = A $op B;
            assert!(result.is_none());
        }
    };
    // Concrete LHS with concrete result (safe operations)
    ($test_name:ident, $TypeA:ty, $a:expr, $op:tt, $TypeB:ty, $b:expr, $ResultType:ty, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result: $ResultType = A $op B;
            assert_eq!(result.get(), $expected);
        }
    };
}

mod test_same_type_arithmetic {
    use super::*;

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_positive_add_positive, PositiveF64, 5.0, +, PositiveF64, 3.0, Option<PositiveF64>, Some(8.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_negative_add_negative, NegativeF64, -5.0, +, NegativeF64, -3.0, Option<NegativeF64>, Some(-8.0));

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_negative_sub_negative, NegativeF64, -10.0, -, NegativeF64, -3.0, FinF64, -7.0);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_negative_sub_negative_positive_result, NegativeF64, -5.0, -, NegativeF64, -10.0, FinF64, 5.0);

    // Original: test_fallible_some! -> returns Some
    test_arith!(test_nonzero_add_nonzero, NonZeroF64, 5.0, +, NonZeroF64, 3.0, Option<NonZeroF64>, Some(8.0));

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_positive_sub_positive, PositiveF64, 10.0, -, PositiveF64, 3.0, FinF64, 7.0);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_positive_sub_positive_negative_result, PositiveF64, 5.0, -, PositiveF64, 10.0, FinF64, -5.0);

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_positive_sub_negative, PositiveF64, 10.0, -, NegativeF64, -3.0, Option<PositiveF64>, Some(13.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_negative_sub_positive, NegativeF64, -10.0, -, PositiveF64, 3.0, Option<NegativeF64>, Some(-13.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_nonzero_mul_nonzero, NonZeroF64, 5.0, *, NonZeroF64, 3.0, Option<NonZeroF64>, Some(15.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_positive_div_positive, PositiveF64, 15.0, /, PositiveF64, 3.0, Option<PositiveF64>, Some(5.0));

    // Original: test_fallible_none! -> returns None
    test_arith!(test_positive_div_by_zero_returns_none, PositiveF64, 15.0, /, PositiveF64, 0.0, Option<PositiveF64>, None);
}

mod test_cross_type_arithmetic {
    use super::*;

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_positive_plus_negative, PositiveF64, 5.0, +, NegativeF64, -3.0, FinF64, 2.0);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_negative_plus_positive, NegativeF64, -5.0, +, PositiveF64, 3.0, FinF64, -2.0);

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_positive_minus_negative, PositiveF64, 10.0, -, NegativeF64, -3.0, Option<PositiveF64>, Some(13.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_negative_minus_positive, NegativeF64, -10.0, -, PositiveF64, 3.0, Option<NegativeF64>, Some(-13.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_positive_mul_negative, PositiveF64, 5.0, *, NegativeF64, -3.0, Option<NegativeF64>, Some(-15.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(
        test_nonzero_positive_div_nonzero_negative,
        NonZeroPositiveF64,
        10.0,
        /,
        NonZeroNegativeF64,
        -2.0,
        Option<NonZeroNegativeF64>,
        Some(-5.0)
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_f32_cross_type, PositiveF32, 5.0, +, NegativeF32, -3.0, FinF32, 2.0);
}

mod test_safe_operations {
    use super::*;

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_normalized_mul_normalized, NormalizedF64, 0.5, *, NormalizedF64, 0.4, PositiveF64, 0.2);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_normalized_mul_negative_normalized,
        NormalizedF64,
        0.5,
        *,
        NegativeNormalizedF64,
        -0.4,
        NegativeF64,
        -0.2
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_negative_normalized_mul_negative_normalized,
        NegativeNormalizedF64,
        -0.5,
        *,
        NegativeNormalizedF64,
        -0.4,
        PositiveF64,
        0.2
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_symmetric_mul_symmetric, SymmetricF64, 0.5, *, SymmetricF64, 0.8, FinF64, 0.4);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_symmetric_mul_symmetric_negative,
        SymmetricF64,
        -0.5,
        *,
        SymmetricF64,
        0.8,
        FinF64,
        -0.4
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_symmetric_mul_symmetric_both_negative,
        SymmetricF64,
        -0.5,
        *,
        SymmetricF64,
        -0.8,
        FinF64,
        0.4
    );
}

mod test_fallible_operations {
    use super::*;

    // Original: test_fallible_none! -> returns None
    test_arith!(test_addition_overflow, PositiveF64, 1e308, +, PositiveF64, 1e308, Option<PositiveF64>, None);

    // Original: test_fallible_none! -> returns None
    test_arith!(test_subtraction_underflow, NegativeF64, -1e308, -, PositiveF64, 1e308, Option<FinF64>, None);

    // Original: test_fallible_none! -> returns None
    test_arith!(test_multiplication_overflow, PositiveF64, 1e200, *, PositiveF64, 1e200, Option<PositiveF64>, None);

    // Original: test_fallible_none! -> returns None
    test_arith!(test_division_by_zero_positive, PositiveF64, 10.0, /, PositiveF64, 0.0, Option<PositiveF64>, None);

    // Original: test_fallible_none! -> returns None
    test_arith!(test_division_by_zero_fin, FinF64, 10.0, /, FinF64, 0.0, Option<FinF64>, None);

    // Original: test_fallible_some! -> returns Some
    test_arith!(test_normalized_add_normalized, NormalizedF64, 0.9, +, NormalizedF64, 0.9, Option<PositiveF64>, Some(1.8));

    // Original: test_fallible_some! -> returns Some
    test_arith!(test_symmetric_add_symmetric, SymmetricF64, 0.9, +, SymmetricF64, 0.9, Option<PositiveF64>, Some(1.8));
}

mod test_option_arithmetic {
    use super::*;

    // Note: These tests require manual implementation because they test Option arithmetic
    // which has different semantics than concrete type arithmetic
    #[test]
    fn test_lhs_plus_option_rhs_some() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Option<NegativeF64> = Some(NegativeF64::new_const(-3.0));
        let result: Option<FinF64> = A + b;
        assert_eq!(result.map(|r| r.get()), Some(2.0));
    }

    #[test]
    fn test_lhs_plus_option_rhs_none() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Option<NegativeF64> = None;
        let result: Option<FinF64> = A + b;
        assert_eq!(result, None);
    }

    #[test]
    fn test_lhs_mul_option_rhs_some() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Option<PositiveF64> = Some(PositiveF64::new_const(3.0));
        let result: Option<PositiveF64> = A * b;
        assert_eq!(result.map(|r| r.get()), Some(15.0));
    }

    #[test]
    fn test_lhs_div_option_rhs_none() {
        const A: PositiveF64 = PositiveF64::new_const(15.0);
        let b: Option<PositiveF64> = None;
        let result: Option<PositiveF64> = A / b;
        assert_eq!(result, None);
    }

    #[test]
    fn test_option_chaining() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        let b: Option<PositiveF64> = Some(PositiveF64::new_const(2.0));
        let c: Option<PositiveF64> = Some(PositiveF64::new_const(3.0));

        // Chain operations with Option
        let result1: Option<PositiveF64> = A + b;
        let result2: Option<PositiveF64> = result1.and_then(|x| x * c);
        assert!(result2.is_some());
        assert_eq!(result2.unwrap().get(), 36.0);
    }

    #[test]
    fn test_option_chaining_with_none() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        let b: Option<PositiveF64> = None;
        let c: Option<PositiveF64> = Some(PositiveF64::new_const(3.0));

        // Chain with None in the middle
        let result1: Option<PositiveF64> = A + b;
        let result2: Option<PositiveF64> = result1.and_then(|x| x * c);
        assert!(result2.is_none());
    }

    #[test]
    fn test_option_division_chain() {
        let a: Option<PositiveF64> = Some(PositiveF64::new_const(100.0));
        let b: Option<PositiveF64> = Some(PositiveF64::new_const(10.0));
        let c: Option<PositiveF64> = Some(PositiveF64::new_const(2.0));

        // Note: We can't do (a / b) / c directly because of orphan rules
        // But we can chain using and_then
        let result: Option<PositiveF64> = match (a, b, c) {
            (Some(x), Some(y), Some(z)) => (x / y).and_then(|quotient| quotient / z),
            _ => None,
        };
        assert!(result.is_some());
        assert_eq!(result.unwrap().get(), 5.0);
    }
}

mod test_combined_constraints {
    use super::*;

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(
        test_nonzero_positive_add_nonzero_positive,
        NonZeroPositiveF64,
        5.0,
        +,
        NonZeroPositiveF64,
        3.0,
        Option<NonZeroPositiveF64>,
        Some(8.0)
    );

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(
        test_nonzero_negative_add_nonzero_negative,
        NonZeroNegativeF64,
        -5.0,
        +,
        NonZeroNegativeF64,
        -3.0,
        Option<NonZeroNegativeF64>,
        Some(-8.0)
    );

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(
        test_nonzero_positive_sub_nonzero_negative,
        NonZeroPositiveF64,
        10.0,
        -,
        NonZeroNegativeF64,
        -3.0,
        Option<NonZeroPositiveF64>,
        Some(13.0)
    );

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(
        test_nonzero_positive_mul_nonzero_negative,
        NonZeroPositiveF64,
        5.0,
        *,
        NonZeroNegativeF64,
        -3.0,
        Option<NonZeroNegativeF64>,
        Some(-15.0)
    );
}

mod test_f32_types {
    use super::*;

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_f32_positive_add_positive, PositiveF32, 5.0, +, PositiveF32, 3.0, Option<PositiveF32>, Some(8.0));

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_f32_cross_type_operations, PositiveF32, 5.0, +, NegativeF32, -3.0, FinF32, 2.0);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_f32_safe_multiplication,
        NormalizedF32,
        0.5,
        *,
        NormalizedF32,
        0.4,
        PositiveF32,
        0.2
    );

    // Original: division by zero test
    test_arith!(test_f32_division_by_zero, PositiveF32, 15.0, /, PositiveF32, 0.0, Option<PositiveF32>, None);
}

mod test_negation_interaction {
    use super::*;

    // Note: These tests use negation operator and cannot be expressed with test_arith macro
    #[test]
    fn test_add_negation() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        const B: PositiveF64 = PositiveF64::new_const(5.0);
        // a + (-b) = a - b
        let neg_b: NegativeF64 = -B;
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = A + neg_b;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_subtraction_via_negation() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        const B: PositiveF64 = PositiveF64::new_const(5.0);
        // a - b = a + (-b)
        let neg_b: NegativeF64 = -B;
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = A + neg_b;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_double_negation() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        let neg_a: NegativeF64 = -A;
        let pos_a: PositiveF64 = -neg_a;
        assert_eq!(pos_a.get(), A.get());
    }
}

mod test_edge_cases {
    use super::*;

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_addition_with_zero, PositiveF64, 5.0, +, PositiveF64, 0.0, Option<PositiveF64>, Some(5.0));

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_subtraction_with_zero, PositiveF64, 5.0, -, PositiveF64, 0.0, FinF64, 5.0);

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_multiplication_with_zero, PositiveF64, 5.0, *, PositiveF64, 0.0, Option<PositiveF64>, Some(0.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_division_with_one, PositiveF64, 5.0, /, PositiveF64, 1.0, Option<PositiveF64>, Some(5.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_symmetric_extremes, SymmetricF64, 1.0, +, SymmetricF64, -1.0, Option<FinF64>, Some(0.0));

    // Original: test_arith! -> returns Option, needs unwrap
    test_arith!(test_normalized_extremes, NormalizedF64, 0.0, +, NormalizedF64, 1.0, Option<PositiveF64>, Some(1.0));
}

mod test_safe_division {
    use super::*;

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_normalized_div_nonzero, NormalizedF64, 0.5, /, NonZeroF64, 2.0, FinF64, 0.25);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_normalized_div_nonzero_positive,
        NormalizedF64,
        1.0,
        /,
        NonZeroPositiveF64,
        2.0,
        PositiveF64,
        0.5
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_normalized_div_nonzero_positive_small,
        NormalizedF64,
        0.8,
        /,
        NonZeroPositiveF64,
        4.0,
        PositiveF64,
        0.2
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_negative_normalized_div_nonzero,
        NegativeNormalizedF64,
        -0.5,
        /,
        NonZeroF64,
        2.0,
        FinF64,
        -0.25
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_negative_normalized_div_nonzero_negative,
        NegativeNormalizedF64,
        -1.0,
        /,
        NonZeroNegativeF64,
        -2.0,
        PositiveF64,
        0.5
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_symmetric_div_nonzero, SymmetricF64, 0.5, /, NonZeroF64, 2.0, FinF64, 0.25);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_symmetric_div_nonzero_positive,
        SymmetricF64,
        -0.8,
        /,
        NonZeroPositiveF64,
        4.0,
        FinF64,
        -0.2
    );

    // Original: test_safe_arith! -> returns direct value
    test_arith!(test_normalized_zero_div_nonzero, NormalizedF64, 0.0, /, NonZeroF64, 5.0, FinF64, 0.0);

    // Original: test_safe_arith! -> returns direct value
    test_arith!(
        test_normalized_div_by_negative_nonzero,
        NormalizedF64,
        0.5,
        /,
        NonZeroNegativeF64,
        -2.0,
        NegativeF64,
        -0.25
    );

    // Note: These tests use f64::MIN/f64::MAX constants and cannot use new_const
    #[test]
    fn test_one_div_f64_min() {
        const A: NormalizedF64 = NormalizedF64::new_const(1.0);
        const B: NonZeroF64 = NonZeroF64::new_const(f64::MIN);
        // Safe operation: 1.0 / f64::MIN ≈ -5.56e-319 (finite, not overflow)
        let result: FinF64 = A / B;
        // Result should be very small but finite
        assert!(result.get().is_finite());
        assert!(result.get() < 0.0); // Should be negative (1.0 / negative = negative)
        assert!(result.get().abs() < 1e-308);
    }

    #[test]
    fn test_negative_one_div_f64_max() {
        const A: NegativeNormalizedF64 = NegativeNormalizedF64::new_const(-1.0);
        const B: NonZeroF64 = NonZeroF64::new_const(f64::MAX);
        // Safe operation: -1.0 / f64::MAX ≈ -5.56e-319 (finite)
        let result: FinF64 = A / B;
        assert!(result.get().is_finite());
        assert!(result.get() < 0.0);
        assert!(result.get().abs() < 1e-308);
    }

    #[test]
    fn test_symmetric_extremes_div() {
        const A: SymmetricF64 = SymmetricF64::new_const(1.0);
        const B: NonZeroPositiveF64 = NonZeroPositiveF64::new_const(f64::MIN_POSITIVE);
        // Safe operation: 1.0 / smallest positive ≈ 8.99e+307 (large but finite)
        let result: FinF64 = A / B;
        assert!(result.get().is_finite());
        assert!(result.get() > 0.0);
        assert!(result.get() > 1e307);
    }
}
