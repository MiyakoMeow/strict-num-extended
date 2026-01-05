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
        let pos = PositiveF64::new(5.0).unwrap();
        let neg: NegativeF64 = -pos;
        assert_eq!(neg.get(), -5.0);
    }

    #[test]
    fn test_negative_to_positive_f32() {
        let neg = NegativeF32::new(-2.5).unwrap();
        let pos: PositiveF32 = -neg;
        assert!((pos.get() - 2.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_negation() {
        let pos_zero = PositiveF32::new(0.0).unwrap();
        let neg_zero: NegativeF32 = -pos_zero;
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
        let nz_pos = NonZeroPositiveF64::new(10.0).unwrap();
        let nz_neg: NonZeroNegativeF64 = -nz_pos;
        assert_eq!(nz_neg.get(), -10.0);
    }

    #[test]
    fn test_nonzero_negative_to_positive() {
        let nz_neg = NonZeroNegativeF32::new(-2.5).unwrap();
        let nz_pos: NonZeroPositiveF32 = -nz_neg;
        assert_eq!(nz_pos.get(), 2.5);
    }

    #[test]
    fn test_double_negation() {
        let original = NonZeroPositiveF32::new(10.0).unwrap();
        let neg1: NonZeroNegativeF32 = -original;
        let back: NonZeroPositiveF32 = -neg1;
        assert_eq!(back.get(), 10.0);
    }
}

/// Tests for Normalized ↔ `NegativeNormalized` conversion
mod test_normalized {
    use super::*;

    #[test]
    fn test_normalized_to_negative_normalized() {
        let norm = NormalizedF64::new(0.75).unwrap();
        let neg_norm: NegativeNormalizedF64 = -norm;
        assert_eq!(neg_norm.get(), -0.75);
    }

    #[test]
    fn test_negative_normalized_to_normalized() {
        let neg_norm = NegativeNormalizedF32::new(-0.5).unwrap();
        let norm: NormalizedF32 = -neg_norm;
        assert_eq!(norm.get(), 0.5);
    }

    #[test]
    fn test_boundary_values() {
        // Test boundary values 0.0 and 1.0
        let zero = NormalizedF32::new(0.0).unwrap();
        let neg_zero: NegativeNormalizedF32 = -zero;
        assert_eq!(neg_zero.get(), -0.0);

        let one = NormalizedF32::new(1.0).unwrap();
        let neg_one: NegativeNormalizedF32 = -one;
        assert_eq!(neg_one.get(), -1.0);

        let neg_minus_one = NegativeNormalizedF32::new(-1.0).unwrap();
        let back_to_one: NormalizedF32 = -neg_minus_one;
        assert_eq!(back_to_one.get(), 1.0);
    }
}

/// Tests for reflexive constraints (Fin, `NonZero`)
mod test_reflexive {
    use super::*;

    #[test]
    fn test_fin_negation_f64() {
        let fin = FinF64::new(2.5).unwrap();
        let neg: FinF64 = -fin;
        assert_eq!(neg.get(), -2.5);
    }

    #[test]
    fn test_fin_negation_f32() {
        let fin = FinF32::new(-1.5).unwrap();
        let neg: FinF32 = -fin;
        assert_eq!(neg.get(), 1.5);
    }

    #[test]
    fn test_nonzero_negation() {
        let nz = NonZeroF32::new(5.0).unwrap();
        let nz_neg: NonZeroF32 = -nz;
        assert_eq!(nz_neg.get(), -5.0);
    }

    #[test]
    fn test_double_fin_negation() {
        let original = FinF64::new(1.414).unwrap();
        let neg1: FinF64 = -original;
        let back: FinF64 = -neg1;
        assert!((back.get() - 1.414).abs() < f64::EPSILON);
    }
}

/// Tests for edge cases
mod test_edge_cases {
    use super::*;

    #[test]
    fn test_large_values() {
        let pos = PositiveF64::new(1e100).unwrap();
        let neg: NegativeF64 = -pos;
        assert_eq!(neg.get(), -1e100);
    }

    #[test]
    fn test_small_values() {
        let pos = PositiveF32::new(1e-30).unwrap();
        let neg: NegativeF32 = -pos;
        assert_eq!(neg.get(), -1e-30);
    }

    #[test]
    fn test_normalized_midpoint() {
        let mid = NormalizedF64::new(0.5).unwrap();
        let neg_mid: NegativeNormalizedF64 = -mid;
        assert_eq!(neg_mid.get(), -0.5);
    }
}
