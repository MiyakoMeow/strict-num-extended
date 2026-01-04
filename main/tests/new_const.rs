//! # `new_const` Method Tests
//!
//! Tests `new_const` method for all types

// Strict floating-point comparisons in test code are justified for verifying new_const functionality
#![expect(clippy::float_cmp)]

use strict_num_extended::*;

/// Tests ``FinF32::new_const``
mod test_finf32_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: FinF32 = FinF32::new_const(1.0);
        assert_eq!(VALID.get(), 1.0);
    }

    #[test]
    fn test_zero() {
        const ZERO: FinF32 = FinF32::new_const(0.0);
        assert_eq!(ZERO.get(), 0.0);
    }

    #[test]
    fn test_pi() {
        const PI: FinF32 = FinF32::new_const(std::f32::consts::PI);
        assert!((PI.get() - std::f32::consts::PI).abs() < f32::EPSILON);
    }

    #[test]
    fn test_negative() {
        const NEG: FinF32 = FinF32::new_const(-1.5);
        assert_eq!(NEG.get(), -1.5);
    }
}

/// Tests ``PositiveF32::new_const``
mod test_positivef32_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: PositiveF32 = PositiveF32::new_const(42.0);
        assert_eq!(VALID.get(), 42.0);
    }

    #[test]
    fn test_small_positive() {
        const SMALL: PositiveF32 = PositiveF32::new_const(0.001);
        assert_eq!(SMALL.get(), 0.001);
    }
}

/// Tests `NegativeF32::new_const`
mod test_negativef32_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: NegativeF32 = NegativeF32::new_const(-42.0);
        assert_eq!(VALID.get(), -42.0);
    }

    #[test]
    fn test_negative_zero() {
        const ZERO: NegativeF32 = NegativeF32::new_const(-0.0);
        assert_eq!(ZERO.get(), -0.0);
    }

    #[test]
    fn test_small_negative() {
        const SMALL: NegativeF32 = NegativeF32::new_const(-0.001);
        assert_eq!(SMALL.get(), -0.001);
    }
}

/// Tests `NonZeroF32::new_const`
mod test_nonzerof32_new_const {
    use super::*;

    #[test]
    fn test_positive_value() {
        const POS: NonZeroF32 = NonZeroF32::new_const(5.0);
        assert_eq!(POS.get(), 5.0);
    }

    #[test]
    fn test_negative_value() {
        const NEG: NonZeroF32 = NonZeroF32::new_const(-5.0);
        assert_eq!(NEG.get(), -5.0);
    }

    #[test]
    fn test_small_positive() {
        const SMALL: NonZeroF32 = NonZeroF32::new_const(0.001);
        assert_eq!(SMALL.get(), 0.001);
    }
}

/// Tests `NonZeroPositiveF32::new_const`
mod test_nonzero_positivef32_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: NonZeroPositiveF32 = NonZeroPositiveF32::new_const(10.0);
        assert_eq!(VALID.get(), 10.0);
    }

    #[test]
    fn test_small_positive() {
        const SMALL: NonZeroPositiveF32 = NonZeroPositiveF32::new_const(0.001);
        assert_eq!(SMALL.get(), 0.001);
    }
}

/// Tests `NonZeroNegativeF32::new_const`
mod test_nonzero_negativef32_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: NonZeroNegativeF32 = NonZeroNegativeF32::new_const(-10.0);
        assert_eq!(VALID.get(), -10.0);
    }

    #[test]
    fn test_small_negative() {
        const SMALL: NonZeroNegativeF32 = NonZeroNegativeF32::new_const(-0.001);
        assert_eq!(SMALL.get(), -0.001);
    }
}

/// Tests `new_const` for f64 types
mod test_f64_new_const {
    use super::*;

    #[test]
    fn test_finf64() {
        const VALUE: FinF64 = FinF64::new_const(std::f64::consts::PI);
        assert!((VALUE.get() - std::f64::consts::PI).abs() < f64::EPSILON);
    }

    #[test]
    fn test_positivef64() {
        const VALUE: PositiveF64 = PositiveF64::new_const(123.456);
        assert_eq!(VALUE.get(), 123.456);
    }

    #[test]
    fn test_negativef64() {
        const VALUE: NegativeF64 = NegativeF64::new_const(-789.012);
        assert_eq!(VALUE.get(), -789.012);
    }

    #[test]
    fn test_nonzerof64() {
        const POS: NonZeroF64 = NonZeroF64::new_const(3.0);
        const NEG: NonZeroF64 = NonZeroF64::new_const(-2.0);
        assert_eq!(POS.get(), 3.0);
        assert_eq!(NEG.get(), -2.0);
    }

    #[test]
    fn test_nonzero_positivef64() {
        const VALUE: NonZeroPositiveF64 = NonZeroPositiveF64::new_const(999.999);
        assert_eq!(VALUE.get(), 999.999);
    }

    #[test]
    fn test_nonzero_negativef64() {
        const VALUE: NonZeroNegativeF64 = NonZeroNegativeF64::new_const(-888.888);
        assert_eq!(VALUE.get(), -888.888);
    }
}

/// Tests boundary values
mod test_boundary_values {
    use super::*;

    #[test]
    fn test_finf32_max() {
        const MAX: FinF32 = FinF32::new_const(f32::MAX);
        assert_eq!(MAX.get(), f32::MAX);
    }

    #[test]
    fn test_finf32_min() {
        const MIN: FinF32 = FinF32::new_const(f32::MIN);
        assert_eq!(MIN.get(), f32::MIN);
    }

    #[test]
    fn test_finf64_max() {
        const MAX: FinF64 = FinF64::new_const(f64::MAX);
        assert_eq!(MAX.get(), f64::MAX);
    }

    #[test]
    fn test_finf64_min() {
        const MIN: FinF64 = FinF64::new_const(f64::MIN);
        assert_eq!(MIN.get(), f64::MIN);
    }

    #[test]
    fn test_positivef32_max() {
        const MAX: PositiveF32 = PositiveF32::new_const(f32::MAX);
        assert_eq!(MAX.get(), f32::MAX);
    }

    #[test]
    fn test_positivef64_max() {
        const MAX: PositiveF64 = PositiveF64::new_const(f64::MAX);
        assert_eq!(MAX.get(), f64::MAX);
    }

    #[test]
    fn test_negativef32_min() {
        const MIN: NegativeF32 = NegativeF32::new_const(f32::MIN);
        assert_eq!(MIN.get(), f32::MIN);
    }

    #[test]
    fn test_negativef64_min() {
        const MIN: NegativeF64 = NegativeF64::new_const(f64::MIN);
        assert_eq!(MIN.get(), f64::MIN);
    }
}

/// Tests `NegativeNormalizedF32::new_const`
mod test_negative_normalizedf32_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(-0.75);
        assert_eq!(VALID.get(), -0.75);
    }

    #[test]
    fn test_boundary_negative_one() {
        const NEG_ONE: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(-1.0);
        assert_eq!(NEG_ONE.get(), -1.0);
    }

    #[test]
    fn test_boundary_zero() {
        const ZERO: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(0.0);
        assert_eq!(ZERO.get(), 0.0);
    }

    #[test]
    fn test_small_negative() {
        const SMALL: NegativeNormalizedF32 = NegativeNormalizedF32::new_const(-0.001);
        assert_eq!(SMALL.get(), -0.001);
    }
}

/// Tests `NegativeNormalizedF64::new_const`
mod test_negative_normalizedf64_new_const {
    use super::*;

    #[test]
    fn test_valid_value() {
        const VALID: NegativeNormalizedF64 = NegativeNormalizedF64::new_const(-0.75);
        assert_eq!(VALID.get(), -0.75);
    }

    #[test]
    fn test_boundary_negative_one() {
        const NEG_ONE: NegativeNormalizedF64 = NegativeNormalizedF64::new_const(-1.0);
        assert_eq!(NEG_ONE.get(), -1.0);
    }

    #[test]
    fn test_boundary_zero() {
        const ZERO: NegativeNormalizedF64 = NegativeNormalizedF64::new_const(0.0);
        assert_eq!(ZERO.get(), 0.0);
    }
}
