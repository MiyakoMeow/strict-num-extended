//! Arithmetic operations tests
//!
//! Tests for type-safe arithmetic operations between different constraint types.

// Strict floating-point comparisons, unwrap usage, and variable shadowing in test code are justified
#![allow(clippy::float_cmp, clippy::unwrap_used, clippy::shadow_unrelated)]

use strict_num_extended::*;

mod test_same_type_arithmetic {
    use super::*;

    #[test]
    fn test_positive_add_positive() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        const B: PositiveF64 = PositiveF64::new_const(3.0);
        let result: PositiveF64 = (A + B).unwrap();
        assert_eq!(result.get(), 8.0);
    }

    #[test]
    fn test_negative_add_negative() {
        const A: NegativeF64 = NegativeF64::new_const(-5.0);
        const B: NegativeF64 = NegativeF64::new_const(-3.0);
        let result: NegativeF64 = (A + B).unwrap();
        assert_eq!(result.get(), -8.0);
    }

    #[test]
    fn test_negative_sub_negative() {
        const A: NegativeF64 = NegativeF64::new_const(-10.0);
        const B: NegativeF64 = NegativeF64::new_const(-3.0);
        // Safe operation: returns direct value (result is Fin, not Option)
        let result: FinF64 = A - B;
        assert_eq!(result.get(), -7.0);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_negative_sub_negative_positive_result() {
        const A: NegativeF64 = NegativeF64::new_const(-5.0);
        const B: NegativeF64 = NegativeF64::new_const(-10.0);
        // Safe operation: returns direct value (result is Fin, not Option)
        // Note: -5.0 - (-10.0) = 5.0 (positive result from Negative - Negative)
        let result: FinF64 = A - B;
        assert_eq!(result.get(), 5.0);
        // Result is Fin (not Negative) because it can be positive
        assert!(result.get() > 0.0);
    }

    #[test]
    fn test_nonzero_add_nonzero() {
        const A: NonZeroF64 = NonZeroF64::new_const(5.0);
        const B: NonZeroF64 = NonZeroF64::new_const(3.0);
        let result = (A + B).unwrap();
        assert_eq!(result.get(), 8.0);
    }

    #[test]
    fn test_positive_sub_positive() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        const B: PositiveF64 = PositiveF64::new_const(3.0);
        // Safe operation: returns direct value (result is Fin, not Option)
        let result: FinF64 = A - B;
        assert_eq!(result.get(), 7.0);
        // Result can be Fin (may be positive or negative)
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_positive_sub_positive_negative_result() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        const B: PositiveF64 = PositiveF64::new_const(10.0);
        // Safe operation: returns direct value (result is Fin, not Option)
        // Note: 5.0 - 10.0 = -5.0 (negative result from Positive - Positive)
        let result: FinF64 = A - B;
        assert_eq!(result.get(), -5.0);
        // Result is Fin (not Positive) because it can be negative
        assert!(result.get() < 0.0);
    }

    #[test]
    fn test_positive_sub_negative() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let b: NegativeF64 = NegativeF64::new(-3.0).unwrap();
        // Unsafe operation: returns Option (Positive - Negative = Positive + Positive)
        let result: PositiveF64 = (a - b).unwrap();
        assert_eq!(result.get(), 13.0);
    }

    #[test]
    fn test_negative_sub_positive() {
        let a: NegativeF64 = NegativeF64::new(-10.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(3.0).unwrap();
        // Unsafe operation: returns Option (Negative - Positive = Negative + Negative)
        let result: NegativeF64 = (a - b).unwrap();
        assert_eq!(result.get(), -13.0);
    }

    #[test]
    fn test_nonzero_mul_nonzero() {
        let a: NonZeroF64 = NonZeroF64::new(5.0).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(3.0).unwrap();
        let result: NonZeroF64 = (a * b).unwrap();
        assert_eq!(result.get(), 15.0);
    }

    #[test]
    fn test_positive_div_positive() {
        let a: PositiveF64 = PositiveF64::new(15.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(3.0).unwrap();
        let result: PositiveF64 = (a / b).unwrap();
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_positive_div_by_zero_returns_none() {
        let a: PositiveF64 = PositiveF64::new(15.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(0.0).unwrap();
        let result: Option<PositiveF64> = a / b;
        assert!(result.is_none());
    }
}

mod test_cross_type_arithmetic {
    use super::*;

    #[test]
    fn test_positive_plus_negative() {
        let pos: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let neg: NegativeF64 = NegativeF64::new(-3.0).unwrap();
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = pos + neg;
        assert_eq!(result.get(), 2.0);
        // Result should be Fin type (not necessarily positive or negative)
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_negative_plus_positive() {
        let neg: NegativeF64 = NegativeF64::new(-5.0).unwrap();
        let pos: PositiveF64 = PositiveF64::new(3.0).unwrap();
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = neg + pos;
        assert_eq!(result.get(), -2.0);
    }

    #[test]
    fn test_positive_minus_negative() {
        let pos: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let neg: NegativeF64 = NegativeF64::new(-3.0).unwrap();
        // Unsafe operation: returns Option (Positive - Negative = Positive + Positive)
        let result: PositiveF64 = (pos - neg).unwrap();
        assert_eq!(result.get(), 13.0);
    }

    #[test]
    fn test_negative_minus_positive() {
        let neg: NegativeF64 = NegativeF64::new(-10.0).unwrap();
        let pos: PositiveF64 = PositiveF64::new(3.0).unwrap();
        // Unsafe operation: returns Option (Negative - Positive = Negative + Negative)
        let result: NegativeF64 = (neg - pos).unwrap();
        assert_eq!(result.get(), -13.0);
    }

    #[test]
    fn test_positive_mul_negative() {
        let pos: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let neg: NegativeF64 = NegativeF64::new(-3.0).unwrap();
        let result: NegativeF64 = (pos * neg).unwrap();
        assert_eq!(result.get(), -15.0);
        // Result should be Negative type
        assert!(result.get() < 0.0);
    }

    #[test]
    fn test_nonzero_positive_div_nonzero_negative() {
        let pos: NonZeroPositiveF64 = NonZeroPositiveF64::new(10.0).unwrap();
        let neg: NonZeroNegativeF64 = NonZeroNegativeF64::new(-2.0).unwrap();
        // Division always returns Option (overflow/underflow possible)
        let result: NonZeroNegativeF64 = (pos / neg).unwrap();
        assert_eq!(result.get(), -5.0);
        // Result should be NonZeroNegative type
        assert!(result.get() < 0.0);
        assert_ne!(result.get(), 0.0);
    }

    #[test]
    fn test_f32_cross_type() {
        let pos: PositiveF32 = PositiveF32::new(5.0).unwrap();
        let neg: NegativeF32 = NegativeF32::new(-3.0).unwrap();
        // Safe operation: returns direct value (not Option)
        let result: FinF32 = pos + neg;
        assert_eq!(result.get(), 2.0);
    }
}

mod test_safe_operations {
    use super::*;

    #[test]
    fn test_normalized_mul_normalized() {
        let a: NormalizedF64 = NormalizedF64::new(0.5).unwrap();
        let b: NormalizedF64 = NormalizedF64::new(0.4).unwrap();
        // Safe operation: returns direct value, not Option
        let result = a * b;
        assert_eq!(result.get(), 0.2);
        // Result should be Normalized (0.0 <= result <= 1.0)
        assert!(result.get() >= 0.0);
        assert!(result.get() <= 1.0);
    }

    #[test]
    fn test_normalized_mul_negative_normalized() {
        let a: NormalizedF64 = NormalizedF64::new(0.5).unwrap();
        let b: NegativeNormalizedF64 = NegativeNormalizedF64::new(-0.4).unwrap();
        let result = a * b;
        assert_eq!(result.get(), -0.2);
        // Result should be NegativeNormalized
        assert!(result.get() <= 0.0);
        assert!(result.get() >= -1.0);
    }

    #[test]
    fn test_negative_normalized_mul_negative_normalized() {
        let a: NegativeNormalizedF64 = NegativeNormalizedF64::new(-0.5).unwrap();
        let b: NegativeNormalizedF64 = NegativeNormalizedF64::new(-0.4).unwrap();
        let result = a * b;
        assert_eq!(result.get(), 0.2);
        // Result should be Normalized (negative × negative = positive)
        assert!(result.get() >= 0.0);
        assert!(result.get() <= 1.0);
    }

    #[test]
    fn test_symmetric_mul_symmetric() {
        let a: SymmetricF64 = SymmetricF64::new(0.5).unwrap();
        let b: SymmetricF64 = SymmetricF64::new(0.8).unwrap();
        let result = a * b;
        assert_eq!(result.get(), 0.4);
        // Result should be Symmetric (-1.0 <= result <= 1.0)
        assert!(result.get() >= -1.0);
        assert!(result.get() <= 1.0);
    }

    #[test]
    fn test_symmetric_mul_symmetric_negative() {
        let a: SymmetricF64 = SymmetricF64::new(-0.5).unwrap();
        let b: SymmetricF64 = SymmetricF64::new(0.8).unwrap();
        let result = a * b;
        assert_eq!(result.get(), -0.4);
        assert!(result.get() >= -1.0);
        assert!(result.get() <= 1.0);
    }

    #[test]
    fn test_symmetric_mul_symmetric_both_negative() {
        let a: SymmetricF64 = SymmetricF64::new(-0.5).unwrap();
        let b: SymmetricF64 = SymmetricF64::new(-0.8).unwrap();
        let result = a * b;
        assert_eq!(result.get(), 0.4);
        assert!(result.get() >= -1.0);
        assert!(result.get() <= 1.0);
    }
}

mod test_fallible_operations {
    use super::*;

    #[test]
    fn test_addition_overflow() {
        let a: PositiveF64 = PositiveF64::new(1e308).unwrap();
        let b: PositiveF64 = PositiveF64::new(1e308).unwrap();
        let result: Option<PositiveF64> = a + b;
        // Should fail due to overflow (result is infinity)
        assert!(result.is_none());
    }

    #[test]
    fn test_subtraction_underflow() {
        let a: NegativeF64 = NegativeF64::new(-1e308).unwrap();
        let b: PositiveF64 = PositiveF64::new(1e308).unwrap();
        let result = a - b;
        // Should fail due to underflow (result is -infinity)
        assert!(result.is_none());
    }

    #[test]
    fn test_multiplication_overflow() {
        let a: PositiveF64 = PositiveF64::new(1e200).unwrap();
        let b: PositiveF64 = PositiveF64::new(1e200).unwrap();
        let result: Option<PositiveF64> = a * b;
        // Should fail due to overflow (result is infinity)
        assert!(result.is_none());
    }

    #[test]
    fn test_division_by_zero_positive() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(0.0).unwrap(); // Positive allows 0.0
        let result: Option<PositiveF64> = a / b;
        // Should fail: dividing by zero (Positive includes zero)
        assert!(result.is_none());
    }

    #[test]
    fn test_division_by_zero_fin() {
        let a: FinF64 = FinF64::new(10.0).unwrap();
        let b: FinF64 = FinF64::new(0.0).unwrap();
        let result: Option<FinF64> = a / b;
        assert!(result.is_none());
    }

    #[test]
    fn test_normalized_add_normalized() {
        let a: NormalizedF64 = NormalizedF64::new(0.9).unwrap();
        let b: NormalizedF64 = NormalizedF64::new(0.9).unwrap();
        let result: Option<PositiveF64> = a + b;
        // Result type is Positive (not Normalized), so 1.8 is valid
        assert!(result.is_some());
        assert_eq!(result.unwrap().get(), 1.8);
    }

    #[test]
    fn test_symmetric_add_symmetric() {
        let a: SymmetricF64 = SymmetricF64::new(0.9).unwrap();
        let b: SymmetricF64 = SymmetricF64::new(0.9).unwrap();
        let result: Option<FinF64> = a + b;
        // Result type is Fin, so 1.8 is valid (within finite range)
        assert!(result.is_some());
        assert_eq!(result.unwrap().get(), 1.8);
    }
}

mod test_option_arithmetic {
    use super::*;

    #[test]
    fn test_lhs_plus_option_rhs_some() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: Option<NegativeF64> = Some(NegativeF64::new(-3.0).unwrap());
        let result: Option<FinF64> = a + b;
        assert!(result.is_some());
        assert_eq!(result.unwrap().get(), 2.0);
    }

    #[test]
    fn test_lhs_plus_option_rhs_none() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: Option<NegativeF64> = None;
        let result: Option<FinF64> = a + b;
        assert!(result.is_none());
    }

    #[test]
    fn test_lhs_mul_option_rhs_some() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: Option<PositiveF64> = Some(PositiveF64::new(3.0).unwrap());
        let result: Option<PositiveF64> = a * b;
        assert!(result.is_some());
        assert_eq!(result.unwrap().get(), 15.0);
    }

    #[test]
    fn test_lhs_div_option_rhs_none() {
        let a: PositiveF64 = PositiveF64::new(15.0).unwrap();
        let b: Option<PositiveF64> = None;
        let result: Option<PositiveF64> = a / b;
        assert!(result.is_none());
    }

    #[test]
    fn test_option_chaining() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let b: Option<PositiveF64> = Some(PositiveF64::new(2.0).unwrap());
        let c: Option<PositiveF64> = Some(PositiveF64::new(3.0).unwrap());

        // Chain operations with Option
        let result1: Option<PositiveF64> = a + b;
        let result2: Option<PositiveF64> = result1.and_then(|x| x * c);
        assert!(result2.is_some());
        assert_eq!(result2.unwrap().get(), 36.0);
    }

    #[test]
    fn test_option_chaining_with_none() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let b: Option<PositiveF64> = None;
        let c: Option<PositiveF64> = Some(PositiveF64::new(3.0).unwrap());

        // Chain with None in the middle
        let result1: Option<PositiveF64> = a + b;
        let result2: Option<PositiveF64> = result1.and_then(|x| x * c);
        assert!(result2.is_none());
    }

    #[test]
    fn test_option_division_chain() {
        let a: Option<PositiveF64> = Some(PositiveF64::new(100.0).unwrap());
        let b: Option<PositiveF64> = Some(PositiveF64::new(10.0).unwrap());
        let c: Option<PositiveF64> = Some(PositiveF64::new(2.0).unwrap());

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

    #[test]
    fn test_nonzero_positive_add_nonzero_positive() {
        let a: NonZeroPositiveF64 = NonZeroPositiveF64::new(5.0).unwrap();
        let b: NonZeroPositiveF64 = NonZeroPositiveF64::new(3.0).unwrap();
        let result: NonZeroPositiveF64 = (a + b).unwrap();
        assert_eq!(result.get(), 8.0);
        assert!(result.get() > 0.0);
    }

    #[test]
    fn test_nonzero_negative_add_nonzero_negative() {
        let a: NonZeroNegativeF64 = NonZeroNegativeF64::new(-5.0).unwrap();
        let b: NonZeroNegativeF64 = NonZeroNegativeF64::new(-3.0).unwrap();
        let result: NonZeroNegativeF64 = (a + b).unwrap();
        assert_eq!(result.get(), -8.0);
        assert!(result.get() < 0.0);
    }

    #[test]
    fn test_nonzero_positive_sub_nonzero_negative() {
        let a: NonZeroPositiveF64 = NonZeroPositiveF64::new(10.0).unwrap();
        let b: NonZeroNegativeF64 = NonZeroNegativeF64::new(-3.0).unwrap();
        // Unsafe operation: returns Option (NonZeroPositive - NonZeroNegative = NonZeroPositive + NonZeroPositive)
        let result: NonZeroPositiveF64 = (a - b).unwrap();
        assert_eq!(result.get(), 13.0);
        assert!(result.get() > 0.0);
    }

    #[test]
    fn test_nonzero_positive_mul_nonzero_negative() {
        let a: NonZeroPositiveF64 = NonZeroPositiveF64::new(5.0).unwrap();
        let b: NonZeroNegativeF64 = NonZeroNegativeF64::new(-3.0).unwrap();
        let result: NonZeroNegativeF64 = (a * b).unwrap();
        assert_eq!(result.get(), -15.0);
        assert!(result.get() < 0.0);
        assert_ne!(result.get(), 0.0);
    }
}

mod test_f32_types {
    use super::*;

    #[test]
    fn test_f32_positive_add_positive() {
        let a: PositiveF32 = PositiveF32::new(5.0).unwrap();
        let b: PositiveF32 = PositiveF32::new(3.0).unwrap();
        let result: PositiveF32 = (a + b).unwrap();
        assert_eq!(result.get(), 8.0);
    }

    #[test]
    fn test_f32_cross_type_operations() {
        let a: PositiveF32 = PositiveF32::new(5.0).unwrap();
        let b: NegativeF32 = NegativeF32::new(-3.0).unwrap();
        // Safe operation: returns direct value (not Option)
        let result: FinF32 = a + b;
        assert_eq!(result.get(), 2.0);
    }

    #[test]
    fn test_f32_safe_multiplication() {
        let a: NormalizedF32 = NormalizedF32::new(0.5).unwrap();
        let b: NormalizedF32 = NormalizedF32::new(0.4).unwrap();
        let result = a * b;
        assert_eq!(result.get(), 0.2);
    }

    #[test]
    fn test_f32_division_by_zero() {
        let a: PositiveF32 = PositiveF32::new(15.0).unwrap();
        let b: PositiveF32 = PositiveF32::new(0.0).unwrap();
        let result: Option<PositiveF32> = a / b;
        assert!(result.is_none());
    }
}

mod test_negation_interaction {
    use super::*;

    #[test]
    fn test_add_negation() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(5.0).unwrap();
        // a + (-b) = a - b
        let neg_b: NegativeF64 = -b;
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = a + neg_b;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_subtraction_via_negation() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(5.0).unwrap();
        // a - b = a + (-b)
        let neg_b: NegativeF64 = -b;
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = a + neg_b;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_double_negation() {
        let a: PositiveF64 = PositiveF64::new(10.0).unwrap();
        let neg_a: NegativeF64 = -a;
        let pos_a: PositiveF64 = -neg_a;
        assert_eq!(pos_a.get(), a.get());
    }
}

mod test_edge_cases {
    use super::*;

    #[test]
    fn test_addition_with_zero() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(0.0).unwrap();
        let result: PositiveF64 = (a + b).unwrap();
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_subtraction_with_zero() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(0.0).unwrap();
        // Safe operation: returns direct value (not Option)
        let result: FinF64 = a - b;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_multiplication_with_zero() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(0.0).unwrap();
        let result: PositiveF64 = (a * b).unwrap();
        assert_eq!(result.get(), 0.0);
    }

    #[test]
    fn test_division_with_one() {
        let a: PositiveF64 = PositiveF64::new(5.0).unwrap();
        let b: PositiveF64 = PositiveF64::new(1.0).unwrap();
        let result: PositiveF64 = (a / b).unwrap();
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_symmetric_extremes() {
        let a: SymmetricF64 = SymmetricF64::new(1.0).unwrap();
        let b: SymmetricF64 = SymmetricF64::new(-1.0).unwrap();
        let result: FinF64 = (a + b).unwrap();
        assert_eq!(result.get(), 0.0);
    }

    #[test]
    fn test_normalized_extremes() {
        let a: NormalizedF64 = NormalizedF64::new(0.0).unwrap();
        let b: NormalizedF64 = NormalizedF64::new(1.0).unwrap();
        let result: PositiveF64 = (a + b).unwrap();
        assert_eq!(result.get(), 1.0);
    }
}

mod test_safe_division {
    use super::*;

    #[test]
    fn test_normalized_div_nonzero() {
        let a: NormalizedF64 = NormalizedF64::new(0.5).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(2.0).unwrap();
        // Safe operation: Normalized (bounded by [0, 1]) / NonZero = safe
        let result: FinF64 = a / b;
        assert_eq!(result.get(), 0.25);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_normalized_div_nonzero_positive() {
        let a: NormalizedF64 = NormalizedF64::new(1.0).unwrap();
        let b: NonZeroPositiveF64 = NonZeroPositiveF64::new(2.0).unwrap();
        // Safe operation: Normalized (bounded by [0, 1]) / NonZeroPositive = safe
        let result = a / b;
        assert_eq!(result.get(), 0.5);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_normalized_div_nonzero_positive_small() {
        let a: NormalizedF64 = NormalizedF64::new(0.8).unwrap();
        let b: NonZeroPositiveF64 = NonZeroPositiveF64::new(4.0).unwrap();
        // Safe operation: Normalized / NonZeroPositive = safe
        let result = a / b;
        assert_eq!(result.get(), 0.2);
        assert!(result.get().is_finite());
        assert!(result.get() > 0.0);
    }

    #[test]
    fn test_negative_normalized_div_nonzero() {
        let a: NegativeNormalizedF64 = NegativeNormalizedF64::new(-0.5).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(2.0).unwrap();
        // Safe operation: NegativeNormalized (bounded by [-1, 0]) / NonZero = safe
        let result: FinF64 = a / b;
        assert_eq!(result.get(), -0.25);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_negative_normalized_div_nonzero_negative() {
        let a: NegativeNormalizedF64 = NegativeNormalizedF64::new(-1.0).unwrap();
        let b: NonZeroNegativeF64 = NonZeroNegativeF64::new(-2.0).unwrap();
        // Safe operation: NegativeNormalized / NonZeroNegative = safe
        let result = a / b;
        assert_eq!(result.get(), 0.5);
        assert!(result.get().is_finite());
        assert!(result.get() > 0.0);
    }

    #[test]
    fn test_symmetric_div_nonzero() {
        let a: SymmetricF64 = SymmetricF64::new(0.5).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(2.0).unwrap();
        // Safe operation: Symmetric (bounded by [-1, 1]) / NonZero = safe
        let result: FinF64 = a / b;
        assert_eq!(result.get(), 0.25);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_symmetric_div_nonzero_positive() {
        let a: SymmetricF64 = SymmetricF64::new(-0.8).unwrap();
        let b: NonZeroPositiveF64 = NonZeroPositiveF64::new(4.0).unwrap();
        // Safe operation: Symmetric / NonZeroPositive = safe
        let result: FinF64 = a / b;
        assert_eq!(result.get(), -0.2);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_normalized_zero_div_nonzero() {
        let a: NormalizedF64 = NormalizedF64::new(0.0).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(5.0).unwrap();
        // Safe operation: 0.0 / non_zero = 0.0
        let result: FinF64 = a / b;
        assert_eq!(result.get(), 0.0);
        assert!(result.get().is_finite());
    }

    #[test]
    fn test_one_div_f64_min() {
        let a: NormalizedF64 = NormalizedF64::new(1.0).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(f64::MIN).unwrap();
        // Safe operation: 1.0 / f64::MIN ≈ -5.56e-319 (finite, not overflow)
        let result: FinF64 = a / b;
        // Result should be very small but finite
        assert!(result.get().is_finite());
        assert!(result.get() < 0.0); // Should be negative (1.0 / negative = negative)
        assert!(result.get().abs() < 1e-308);
    }

    #[test]
    fn test_negative_one_div_f64_max() {
        let a: NegativeNormalizedF64 = NegativeNormalizedF64::new(-1.0).unwrap();
        let b: NonZeroF64 = NonZeroF64::new(f64::MAX).unwrap();
        // Safe operation: -1.0 / f64::MAX ≈ -5.56e-319 (finite)
        let result: FinF64 = a / b;
        assert!(result.get().is_finite());
        assert!(result.get() < 0.0);
        assert!(result.get().abs() < 1e-308);
    }

    #[test]
    fn test_symmetric_extremes_div() {
        let a: SymmetricF64 = SymmetricF64::new(1.0).unwrap();
        let b: NonZeroPositiveF64 = NonZeroPositiveF64::new(f64::MIN_POSITIVE).unwrap();
        // Safe operation: 1.0 / smallest positive ≈ 8.99e+307 (large but finite)
        let result: FinF64 = a / b;
        assert!(result.get().is_finite());
        assert!(result.get() > 0.0);
        assert!(result.get() > 1e307);
    }

    #[test]
    fn test_normalized_div_by_negative_nonzero() {
        let a: NormalizedF64 = NormalizedF64::new(0.5).unwrap();
        let b: NonZeroNegativeF64 = NonZeroNegativeF64::new(-2.0).unwrap();
        // Safe operation: Normalized / NonZeroNegative = safe
        let result = a / b;
        assert_eq!(result.get(), -0.25);
        assert!(result.get().is_finite());
        assert!(result.get() < 0.0);
    }
}
