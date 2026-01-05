//! Arithmetic operations tests
//!
//! Tests for type-safe arithmetic operations between different constraint types.

// Strict floating-point comparisons, unwrap usage, and variable shadowing in test code are justified
#![allow(clippy::float_cmp, clippy::unwrap_used, clippy::shadow_unrelated)]

use strict_num_extended::*;

/// Macro for testing arithmetic operations with exact comparison
macro_rules! test_arith {
    ($test_name:ident, $TypeA:ty, $op:tt, $TypeB:ty, $ResultType:ty, $a:expr, $b:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result: $ResultType = (A $op B).unwrap();
            assert_eq!(result.get(), $expected);
        }
    };
}

/// Macro for testing safe arithmetic operations (return direct value, not Option)
macro_rules! test_safe_arith {
    ($test_name:ident, $TypeA:ty, $op:tt, $TypeB:ty, $a:expr, $b:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = A $op B;
            assert_eq!(result.get(), $expected);
        }
    };
}

/// Macro for testing arithmetic operations that return None (failure cases)
macro_rules! test_fallible_none {
    ($test_name:ident, $TypeA:ty, $op:tt, $TypeB:ty, $a:expr, $b:expr) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = A $op B;
            assert!(result.is_none());
        }
    };
}

/// Macro for testing arithmetic operations that return Some (success cases)
macro_rules! test_fallible_some {
    ($test_name:ident, $TypeA:ty, $op:tt, $TypeB:ty, $a:expr, $b:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            const B: $TypeB = <$TypeB>::new_const($b);
            let result = A $op B;
            assert!(result.is_some());
            assert_eq!(result.unwrap().get(), $expected);
        }
    };
}

/// Macro for testing Option arithmetic (LHS with Option RHS)
macro_rules! test_option_arith {
    ($test_name:ident, $TypeA:ty, $op:tt, $TypeB:ty, $a:expr, $b_opt:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            let b: Option<$TypeB> = $b_opt;
            let result = A $op b;
            assert_eq!(result.map(|r| r.get()), $expected);
        }
    };
}

mod test_same_type_arithmetic {
    use super::*;

    test_arith!(test_positive_add_positive, PositiveF64, +, PositiveF64, PositiveF64, 5.0, 3.0, 8.0);

    test_arith!(test_negative_add_negative, NegativeF64, +, NegativeF64, NegativeF64, -5.0, -3.0, -8.0);
    test_safe_arith!(test_negative_sub_negative, NegativeF64, -, NegativeF64, -10.0, -3.0, -7.0);
    test_safe_arith!(test_negative_sub_negative_positive_result, NegativeF64, -, NegativeF64, -5.0, -10.0, 5.0);
    test_fallible_some!(test_nonzero_add_nonzero, NonZeroF64, +, NonZeroF64, 5.0, 3.0, 8.0);
    test_safe_arith!(test_positive_sub_positive, PositiveF64, -, PositiveF64, 10.0, 3.0, 7.0);
    test_safe_arith!(test_positive_sub_positive_negative_result, PositiveF64, -, PositiveF64, 5.0, 10.0, -5.0);
    test_arith!(test_positive_sub_negative, PositiveF64, -, NegativeF64, PositiveF64, 10.0, -3.0, 13.0);
    test_arith!(test_negative_sub_positive, NegativeF64, -, PositiveF64, NegativeF64, -10.0, 3.0, -13.0);
    test_arith!(test_nonzero_mul_nonzero, NonZeroF64, *, NonZeroF64, NonZeroF64, 5.0, 3.0, 15.0);
    test_arith!(test_positive_div_positive, PositiveF64, /, PositiveF64, PositiveF64, 15.0, 3.0, 5.0);

    #[test]
    fn test_positive_div_by_zero_returns_none() {
        const A: PositiveF64 = PositiveF64::new_const(15.0);
        const B: PositiveF64 = PositiveF64::new_const(0.0);
        let result: Option<PositiveF64> = A / B;
        assert!(result.is_none());
    }
}

mod test_cross_type_arithmetic {
    use super::*;

    test_safe_arith!(test_positive_plus_negative, PositiveF64, +, NegativeF64, 5.0, -3.0, 2.0);
    test_safe_arith!(test_negative_plus_positive, NegativeF64, +, PositiveF64, -5.0, 3.0, -2.0);
    test_arith!(test_positive_minus_negative, PositiveF64, -, NegativeF64, PositiveF64, 10.0, -3.0, 13.0);
    test_arith!(test_negative_minus_positive, NegativeF64, -, PositiveF64, NegativeF64, -10.0, 3.0, -13.0);
    test_arith!(test_positive_mul_negative, PositiveF64, *, NegativeF64, NegativeF64, 5.0, -3.0, -15.0);
    test_arith!(
        test_nonzero_positive_div_nonzero_negative,
        NonZeroPositiveF64,
        /,
        NonZeroNegativeF64,
        NonZeroNegativeF64,
        10.0,
        -2.0,
        -5.0
    );
    test_safe_arith!(test_f32_cross_type, PositiveF32, +, NegativeF32, 5.0, -3.0, 2.0);
}

mod test_safe_operations {
    use super::*;

    test_safe_arith!(test_normalized_mul_normalized, NormalizedF64, *, NormalizedF64, 0.5, 0.4, 0.2);
    test_safe_arith!(
        test_normalized_mul_negative_normalized,
        NormalizedF64,
        *,
        NegativeNormalizedF64,
        0.5,
        -0.4,
        -0.2
    );
    test_safe_arith!(
        test_negative_normalized_mul_negative_normalized,
        NegativeNormalizedF64,
        *,
        NegativeNormalizedF64,
        -0.5,
        -0.4,
        0.2
    );
    test_safe_arith!(test_symmetric_mul_symmetric, SymmetricF64, *, SymmetricF64, 0.5, 0.8, 0.4);
    test_safe_arith!(
        test_symmetric_mul_symmetric_negative,
        SymmetricF64,
        *,
        SymmetricF64,
        -0.5,
        0.8,
        -0.4
    );
    test_safe_arith!(
        test_symmetric_mul_symmetric_both_negative,
        SymmetricF64,
        *,
        SymmetricF64,
        -0.5,
        -0.8,
        0.4
    );
}

mod test_fallible_operations {
    use super::*;

    test_fallible_none!(test_addition_overflow, PositiveF64, +, PositiveF64, 1e308, 1e308);
    test_fallible_none!(test_subtraction_underflow, NegativeF64, -, PositiveF64, -1e308, 1e308);
    test_fallible_none!(test_multiplication_overflow, PositiveF64, *, PositiveF64, 1e200, 1e200);
    test_fallible_none!(test_division_by_zero_positive, PositiveF64, /, PositiveF64, 10.0, 0.0);
    test_fallible_none!(test_division_by_zero_fin, FinF64, /, FinF64, 10.0, 0.0);
    test_fallible_some!(test_normalized_add_normalized, NormalizedF64, +, NormalizedF64, 0.9, 0.9, 1.8);
    test_fallible_some!(test_symmetric_add_symmetric, SymmetricF64, +, SymmetricF64, 0.9, 0.9, 1.8);
}

mod test_option_arithmetic {
    use super::*;

    test_option_arith!(
        test_lhs_plus_option_rhs_some,
        PositiveF64,
        +,
        NegativeF64,
        5.0,
        Some(NegativeF64::new_const(-3.0)),
        Some(2.0)
    );
    test_option_arith!(test_lhs_plus_option_rhs_none, PositiveF64, +, NegativeF64, 5.0, None, None);
    test_option_arith!(
        test_lhs_mul_option_rhs_some,
        PositiveF64,
        *,
        PositiveF64,
        5.0,
        Some(PositiveF64::new_const(3.0)),
        Some(15.0)
    );
    test_option_arith!(test_lhs_div_option_rhs_none, PositiveF64, /, PositiveF64, 15.0, None, None);

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

    test_arith!(
        test_nonzero_positive_add_nonzero_positive,
        NonZeroPositiveF64,
        +,
        NonZeroPositiveF64,
        NonZeroPositiveF64,
        5.0,
        3.0,
        8.0
    );
    test_arith!(
        test_nonzero_negative_add_nonzero_negative,
        NonZeroNegativeF64,
        +,
        NonZeroNegativeF64,
        NonZeroNegativeF64,
        -5.0,
        -3.0,
        -8.0
    );
    test_arith!(
        test_nonzero_positive_sub_nonzero_negative,
        NonZeroPositiveF64,
        -,
        NonZeroNegativeF64,
        NonZeroPositiveF64,
        10.0,
        -3.0,
        13.0
    );
    test_arith!(
        test_nonzero_positive_mul_nonzero_negative,
        NonZeroPositiveF64,
        *,
        NonZeroNegativeF64,
        NonZeroNegativeF64,
        5.0,
        -3.0,
        -15.0
    );
}

mod test_f32_types {
    use super::*;

    test_arith!(test_f32_positive_add_positive, PositiveF32, +, PositiveF32, PositiveF32, 5.0, 3.0, 8.0);
    test_safe_arith!(test_f32_cross_type_operations, PositiveF32, +, NegativeF32, 5.0, -3.0, 2.0);
    test_safe_arith!(
        test_f32_safe_multiplication,
        NormalizedF32,
        *,
        NormalizedF32,
        0.5,
        0.4,
        0.2
    );

    #[test]
    fn test_f32_division_by_zero() {
        const A: PositiveF32 = PositiveF32::new_const(15.0);
        const B: PositiveF32 = PositiveF32::new_const(0.0);
        let result: Option<PositiveF32> = A / B;
        assert!(result.is_none());
    }
}

mod test_negation_interaction {
    use super::*;

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

    test_arith!(test_addition_with_zero, PositiveF64, +, PositiveF64, PositiveF64, 5.0, 0.0, 5.0);
    test_safe_arith!(test_subtraction_with_zero, PositiveF64, -, PositiveF64, 5.0, 0.0, 5.0);
    test_arith!(test_multiplication_with_zero, PositiveF64, *, PositiveF64, PositiveF64, 5.0, 0.0, 0.0);
    test_arith!(test_division_with_one, PositiveF64, /, PositiveF64, PositiveF64, 5.0, 1.0, 5.0);
    test_arith!(test_symmetric_extremes, SymmetricF64, +, SymmetricF64, FinF64, 1.0, -1.0, 0.0);
    test_arith!(test_normalized_extremes, NormalizedF64, +, NormalizedF64, PositiveF64, 0.0, 1.0, 1.0);
}

mod test_safe_division {
    use super::*;

    test_safe_arith!(test_normalized_div_nonzero, NormalizedF64, /, NonZeroF64, 0.5, 2.0, 0.25);
    test_safe_arith!(
        test_normalized_div_nonzero_positive,
        NormalizedF64,
        /,
        NonZeroPositiveF64,
        1.0,
        2.0,
        0.5
    );
    test_safe_arith!(
        test_normalized_div_nonzero_positive_small,
        NormalizedF64,
        /,
        NonZeroPositiveF64,
        0.8,
        4.0,
        0.2
    );
    test_safe_arith!(
        test_negative_normalized_div_nonzero,
        NegativeNormalizedF64,
        /,
        NonZeroF64,
        -0.5,
        2.0,
        -0.25
    );
    test_safe_arith!(
        test_negative_normalized_div_nonzero_negative,
        NegativeNormalizedF64,
        /,
        NonZeroNegativeF64,
        -1.0,
        -2.0,
        0.5
    );
    test_safe_arith!(test_symmetric_div_nonzero, SymmetricF64, /, NonZeroF64, 0.5, 2.0, 0.25);
    test_safe_arith!(
        test_symmetric_div_nonzero_positive,
        SymmetricF64,
        /,
        NonZeroPositiveF64,
        -0.8,
        4.0,
        -0.2
    );
    test_safe_arith!(test_normalized_zero_div_nonzero, NormalizedF64, /, NonZeroF64, 0.0, 5.0, 0.0);
    test_safe_arith!(
        test_normalized_div_by_negative_nonzero,
        NormalizedF64,
        /,
        NonZeroNegativeF64,
        0.5,
        -2.0,
        -0.25
    );

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
