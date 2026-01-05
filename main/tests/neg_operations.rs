//! # Negation Operation Tests
//!
//! Comprehensive test of negation operation type inference and constraint validation

// Strict floating-point comparisons and unwrap usage in test code are justified
#![allow(clippy::float_cmp, clippy::unwrap_used)]

use strict_num_extended::*;

/// Tests for Positive ↔ Negative conversion
mod test_positive_negative {
    use super::*;

    #[test]
    fn test_positive_to_negative_f64() {
        const POS: PositiveF64 = PositiveF64::new_const(5.0);
        let neg: NegativeF64 = -POS;
        assert_eq!(neg.get(), -5.0);
    }

    #[test]
    fn test_negative_to_positive_f32() {
        const NEG: NegativeF32 = NegativeF32::new_const(-2.5);
        let pos: PositiveF32 = -NEG;
        assert!((pos.get() - 2.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_negation() {
        const POS_ZERO: PositiveF32 = PositiveF32::new_const(0.0);
        let neg_zero: NegativeF32 = -POS_ZERO;
        assert_eq!(neg_zero.get(), -0.0);

        // Negate again to return to zero
        let back: PositiveF32 = -neg_zero;
        assert_eq!(back.get(), 0.0);
    }
}

/// Tests for `NonZeroPositive` ↔ `NonZeroNegative` conversion
mod test_nonzero_positive_negative {
    use super::*;

    #[test]
    fn test_nonzero_positive_to_negative() {
        const NZ_POS: NonZeroPositiveF64 = NonZeroPositiveF64::new_const(10.0);
        let nz_neg: NonZeroNegativeF64 = -NZ_POS;
        assert_eq!(nz_neg.get(), -10.0);
    }

    #[test]
    fn test_nonzero_negative_to_positive() {
        const NZ_NEG: NonZeroNegativeF32 = NonZeroNegativeF32::new_const(-2.5);
        let nz_pos: NonZeroPositiveF32 = -NZ_NEG;
        assert_eq!(nz_pos.get(), 2.5);
    }

    #[test]
    fn test_double_negation() {
        const ORIGINAL: NonZeroPositiveF32 = NonZeroPositiveF32::new_const(10.0);
        let neg1: NonZeroNegativeF32 = -ORIGINAL;
        let back: NonZeroPositiveF32 = -neg1;
        assert_eq!(back.get(), 10.0);
    }
}

/// Tests for Normalized ↔ `NegativeNormalized` conversion
mod test_normalized {
    use super::*;

    #[test]
    fn test_normalized_to_negative_normalized() {
        const NORM: NormalizedF64 = NormalizedF64::new_const(0.75);
        let neg_norm: NegativeNormalizedF64 = -NORM;
        assert_eq!(neg_norm.get(), -0.75);
    }

    #[test]
    fn test_negative_normalized_to_normalized() {
        const NEG_NORM: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(-0.5);
        let norm: NormalizedF32 = -NEG_NORM;
        assert_eq!(norm.get(), 0.5);
    }

    #[test]
    fn test_boundary_values() {
        // Test boundary values 0.0 and 1.0
        const ZERO: NormalizedF32 = NormalizedF32::new_const(0.0);
        let neg_zero: NegativeNormalizedF32 = -ZERO;
        assert_eq!(neg_zero.get(), -0.0);

        const ONE: NormalizedF32 = NormalizedF32::new_const(1.0);
        let neg_one: NegativeNormalizedF32 = -ONE;
        assert_eq!(neg_one.get(), -1.0);

        const NEG_MINUS_ONE: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(-1.0);
        let back_to_one: NormalizedF32 = -NEG_MINUS_ONE;
        assert_eq!(back_to_one.get(), 1.0);
    }
}

/// Tests for reflexive constraints (Fin, `NonZero`)
mod test_reflexive {
    use super::*;

    #[test]
    fn test_fin_negation_f64() {
        const FIN: FinF64 = FinF64::new_const(2.5);
        let neg: FinF64 = -FIN;
        assert_eq!(neg.get(), -2.5);
    }

    #[test]
    fn test_fin_negation_f32() {
        const FIN: FinF32 = FinF32::new_const(-1.5);
        let neg: FinF32 = -FIN;
        assert_eq!(neg.get(), 1.5);
    }

    #[test]
    fn test_nonzero_negation() {
        const NZ: NonZeroF32 = NonZeroF32::new_const(5.0);
        let nz_neg: NonZeroF32 = -NZ;
        assert_eq!(nz_neg.get(), -5.0);
    }

    #[test]
    fn test_double_fin_negation() {
        const ORIGINAL: FinF64 = FinF64::new_const(1.414);
        let neg1: FinF64 = -ORIGINAL;
        let back: FinF64 = -neg1;
        assert!((back.get() - 1.414).abs() < f64::EPSILON);
    }
}

/// Tests for edge cases
mod test_edge_cases {
    use super::*;

    #[test]
    fn test_large_values() {
        const POS: PositiveF64 = PositiveF64::new_const(1e100);
        let neg: NegativeF64 = -POS;
        assert_eq!(neg.get(), -1e100);
    }

    #[test]
    fn test_small_values() {
        const POS: PositiveF32 = PositiveF32::new_const(1e-30);
        let neg: NegativeF32 = -POS;
        assert_eq!(neg.get(), -1e-30);
    }

    #[test]
    fn test_normalized_midpoint() {
        const MID: NormalizedF64 = NormalizedF64::new_const(0.5);
        let neg_mid: NegativeNormalizedF64 = -MID;
        assert_eq!(neg_mid.get(), -0.5);
    }
}
