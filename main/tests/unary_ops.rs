//! # Unary Operation Tests
//!
//! Comprehensive test of `abs()` and `signum()` operations with type inference

// Strict floating-point comparisons and unwrap usage in test code are justified
#![allow(clippy::unwrap_used)]

use strict_num_extended::*;

// ============================================================================
// abs() Operation Tests
// ============================================================================

/// Macro for testing `abs()` operations with exact comparison
macro_rules! test_abs {
    ($test_name:ident, $InputType:ty, $input_value:expr, $OutputType:ty, $expected_value:expr) => {
        #[test]
        fn $test_name() {
            const INPUT: $InputType = <$InputType>::new_const($input_value);
            let output: $OutputType = INPUT.abs();
            assert_eq!(output.get(), $expected_value);
        }
    };
}

/// Macro for testing `abs()` operations with floating-point tolerance
macro_rules! test_abs_approx {
    ($test_name:ident, $InputType:ty, $input_value:expr, $OutputType:ty, $expected_value:expr, $eps:ty) => {
        #[test]
        fn $test_name() {
            const INPUT: $InputType = <$InputType>::new_const($input_value);
            let output: $OutputType = INPUT.abs();
            assert!((output.get() - $expected_value).abs() < <$eps>::EPSILON);
        }
    };
}

// Positive → Positive (reflexive)
mod test_abs_positive {
    use super::*;

    test_abs!(test_abs_positive_f64, PositiveF64, 5.0, PositiveF64, 5.0);
    test_abs_approx!(
        test_abs_positive_f32,
        PositiveF32,
        2.5,
        PositiveF32,
        2.5,
        f32
    );

    #[test]
    fn test_abs_positive_zero() {
        const ZERO: PositiveF64 = PositiveF64::new_const(0.0);
        let abs_val: PositiveF64 = ZERO.abs();
        assert_eq!(abs_val.get(), 0.0);
    }
}

// Negative → Positive
mod test_abs_negative {
    use super::*;

    test_abs!(test_abs_negative_f64, NegativeF64, -5.0, PositiveF64, 5.0);
    test_abs_approx!(
        test_abs_negative_f32,
        NegativeF32,
        -2.5,
        PositiveF32,
        2.5,
        f32
    );

    #[test]
    fn test_abs_negative_zero() {
        const ZERO: NegativeF64 = NegativeF64::new_const(0.0);
        let abs_val: PositiveF64 = ZERO.abs();
        assert_eq!(abs_val.get(), 0.0);
    }
}

// NonZero → NonZeroPositive
mod test_abs_nonzero {
    use super::*;

    test_abs!(
        test_abs_nonzero_negative,
        NonZeroF64,
        -5.0,
        NonZeroPositiveF64,
        5.0
    );
    test_abs!(
        test_abs_nonzero_positive,
        NonZeroF64,
        3.0,
        NonZeroPositiveF64,
        3.0
    );

    #[test]
    fn test_abs_nonzero_small() {
        let val = NonZeroF32::new(-1e-10).unwrap();
        let abs_val: NonZeroPositiveF32 = val.abs();
        assert_eq!(abs_val.get(), 1e-10);
    }
}

// Normalized → Normalized (reflexive)
mod test_abs_normalized {
    use super::*;

    test_abs!(
        test_abs_normalized_f64,
        NormalizedF64,
        0.75,
        NormalizedF64,
        0.75
    );
    test_abs_approx!(
        test_abs_normalized_f32,
        NormalizedF32,
        0.333,
        NormalizedF32,
        0.333,
        f32
    );

    #[test]
    fn test_abs_normalized_boundaries() {
        const ZERO: NormalizedF64 = NormalizedF64::new_const(0.0);
        const ONE: NormalizedF64 = NormalizedF64::new_const(1.0);
        assert_eq!(ZERO.abs().get(), 0.0);
        assert_eq!(ONE.abs().get(), 1.0);
    }
}

// NegativeNormalized → Normalized
mod test_abs_negative_normalized {
    use super::*;

    test_abs!(
        test_abs_negative_normalized_f64,
        NegativeNormalizedF64,
        -0.75,
        NormalizedF64,
        0.75
    );
    test_abs_approx!(
        test_abs_negative_normalized_f32,
        NegativeNormalizedF32,
        -0.333,
        NormalizedF32,
        0.333,
        f32
    );

    #[test]
    fn test_abs_negative_normalized_boundaries() {
        const NEG_ONE: NegativeNormalizedF64 = NegativeNormalizedF64::new_const(-1.0);
        const ZERO: NegativeNormalizedF64 = NegativeNormalizedF64::new_const(0.0);
        assert_eq!(NEG_ONE.abs().get(), 1.0);
        assert_eq!(ZERO.abs().get(), 0.0);
    }
}

// NonZeroPositive → NonZeroPositive (reflexive)
mod test_abs_nonzero_positive {
    use super::*;

    test_abs!(
        test_abs_nonzero_positive_f64,
        NonZeroPositiveF64,
        10.0,
        NonZeroPositiveF64,
        10.0
    );
    test_abs!(
        test_abs_nonzero_positive_small,
        NonZeroPositiveF32,
        0.001,
        NonZeroPositiveF32,
        0.001
    );
}

// NonZeroNegative → NonZeroPositive
mod test_abs_nonzero_negative {
    use super::*;

    test_abs!(
        test_abs_nonzero_negative_f64,
        NonZeroNegativeF64,
        -10.0,
        NonZeroPositiveF64,
        10.0
    );
    test_abs!(
        test_abs_nonzero_negative_small,
        NonZeroNegativeF32,
        -0.001,
        NonZeroPositiveF32,
        0.001
    );
}

// Symmetric → Normalized
mod test_abs_symmetric {
    use super::*;

    test_abs!(
        test_abs_symmetric_negative,
        SymmetricF64,
        -0.75,
        NormalizedF64,
        0.75
    );
    test_abs!(
        test_abs_symmetric_positive,
        SymmetricF32,
        0.75,
        NormalizedF32,
        0.75
    );

    #[test]
    fn test_abs_symmetric_boundaries() {
        const NEG_ONE: SymmetricF64 = SymmetricF64::new_const(-1.0);
        const ZERO: SymmetricF64 = SymmetricF64::new_const(0.0);
        const ONE: SymmetricF64 = SymmetricF64::new_const(1.0);

        let abs_neg_one: NormalizedF64 = NEG_ONE.abs();
        let abs_zero: NormalizedF64 = ZERO.abs();
        let abs_one: NormalizedF64 = ONE.abs();

        assert_eq!(abs_neg_one.get(), 1.0);
        assert_eq!(abs_zero.get(), 0.0);
        assert_eq!(abs_one.get(), 1.0);
    }
}

// Fin → Positive
mod test_abs_fin {
    use super::*;

    test_abs!(test_abs_fin_negative, FinF64, -2.5, PositiveF64, 2.5);
    test_abs!(test_abs_fin_positive, FinF32, 1.5, PositiveF32, 1.5);

    #[test]
    fn test_abs_fin_large_values() {
        const LARGE_NEG: FinF64 = FinF64::new_const(-1e100);
        const LARGE_POS: FinF64 = FinF64::new_const(1e100);

        let abs_neg: PositiveF64 = LARGE_NEG.abs();
        let abs_pos: PositiveF64 = LARGE_POS.abs();

        assert_eq!(abs_neg.get(), 1e100);
        assert_eq!(abs_pos.get(), 1e100);
    }

    #[test]
    fn test_abs_fin_zero() {
        const ZERO: FinF64 = FinF64::new_const(0.0);
        let abs_val: PositiveF64 = ZERO.abs();
        assert_eq!(abs_val.get(), 0.0);
    }
}

// ============================================================================
// signum() Operation Tests
// ============================================================================

/// Macro for testing `signum()` operations with type inference
macro_rules! test_signum {
    ($test_name:ident, $InputType:ty, $input_value:expr, $OutputType:ty, $expected_value:expr) => {
        #[test]
        fn $test_name() {
            const INPUT: $InputType = <$InputType>::new_const($input_value);
            let output: $OutputType = INPUT.signum();
            assert_eq!(output.get(), $expected_value);
        }
    };
}

// Test type inference for signum
mod test_signum_type_inference {
    use super::*;

    // Positive types → Normalized (signum in {0, 1})
    test_signum!(
        test_signum_positive_to_normalized,
        PositiveF64,
        5.0,
        NormalizedF64,
        1.0
    );
    test_signum!(
        test_signum_positive_zero,
        PositiveF64,
        0.0,
        NormalizedF64,
        1.0
    );

    test_signum!(
        test_signum_normalized_reflexive,
        NormalizedF64,
        0.75,
        NormalizedF64,
        1.0
    );
    test_signum!(
        test_signum_normalized_zero,
        NormalizedF64,
        0.0,
        NormalizedF64,
        1.0
    );

    test_signum!(
        test_signum_nonzero_positive_to_normalized,
        NonZeroPositiveF64,
        1e5,
        NormalizedF64,
        1.0
    );

    // Negative types → NegativeNormalized (signum in {-1, 0})
    test_signum!(
        test_signum_negative_to_negative_normalized,
        NegativeF64,
        -5.0,
        NegativeNormalizedF64,
        -1.0
    );
    test_signum!(
        test_signum_negative_zero_to_negative_normalized,
        NegativeF64,
        0.0,
        NegativeNormalizedF64,
        1.0
    );

    test_signum!(
        test_signum_negative_normalized_reflexive,
        NegativeNormalizedF64,
        -0.25,
        NegativeNormalizedF64,
        -1.0
    );
    test_signum!(
        test_signum_negative_normalized_zero,
        NegativeNormalizedF64,
        0.0,
        NegativeNormalizedF64,
        1.0
    );

    test_signum!(
        test_signum_nonzero_negative_to_negative_normalized,
        NonZeroNegativeF64,
        -1e5,
        NegativeNormalizedF64,
        -1.0
    );

    // Any sign types → Symmetric
    test_signum!(test_signum_fin_positive, FinF64, 100.0, SymmetricF64, 1.0);
    test_signum!(test_signum_fin_negative, FinF64, -100.0, SymmetricF64, -1.0);
    test_signum!(test_signum_fin_zero, FinF64, 0.0, SymmetricF64, 1.0);

    test_signum!(
        test_signum_nonzero_positive,
        NonZeroF64,
        1e-10,
        SymmetricF64,
        1.0
    );
    test_signum!(
        test_signum_nonzero_negative,
        NonZeroF64,
        -1e-10,
        SymmetricF64,
        -1.0
    );

    test_signum!(
        test_signum_symmetric_positive,
        SymmetricF64,
        0.5,
        SymmetricF64,
        1.0
    );
    test_signum!(
        test_signum_symmetric_negative,
        SymmetricF64,
        -0.5,
        SymmetricF64,
        -1.0
    );
    test_signum!(
        test_signum_symmetric_zero,
        SymmetricF64,
        0.0,
        SymmetricF64,
        1.0
    );
}

// signum() with f32 types
mod test_signum_f32 {
    use super::*;

    #[test]
    fn test_signum_f32_type_inference() {
        const POS: PositiveF32 = PositiveF32::new_const(5.0);
        const NEG: NegativeF32 = NegativeF32::new_const(-5.0);
        const ZERO: FinF32 = FinF32::new_const(0.0);

        // Type inference works for f32 too
        let sign_pos: NormalizedF32 = POS.signum();
        let sign_neg: NegativeNormalizedF32 = NEG.signum();
        let sign_zero: SymmetricF32 = ZERO.signum();

        assert_eq!(sign_pos.get(), 1.0);
        assert_eq!(sign_neg.get(), -1.0);
        assert_eq!(sign_zero.get(), 1.0);
    }
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_abs_and_signum_combined() {
    // Test that signum(abs(x)) == 1.0 for all x ≠ 0
    const NEG_VAL: NegativeF64 = NegativeF64::new_const(-5.0);
    let abs_val: PositiveF64 = NEG_VAL.abs();
    let sign: NormalizedF64 = abs_val.signum();
    assert_eq!(sign.get(), 1.0);

    // Test that abs(x) * signum(x) == x for all x
    const VAL: FinF64 = FinF64::new_const(-3.5);
    let abs_of_val: PositiveF64 = VAL.abs();
    let sign_val: SymmetricF64 = VAL.signum();
    // Note: Can't multiply Positive and Symmetric directly, but we can verify the logic
    assert_eq!(abs_of_val.get(), 3.5);
    assert_eq!(sign_val.get(), -1.0);
}

#[test]
fn test_abs_idempotent() {
    // abs(abs(x)) == abs(x) for all x
    const SYM: SymmetricF64 = SymmetricF64::new_const(-0.75);
    let abs1: NormalizedF64 = SYM.abs();
    let abs2: NormalizedF64 = abs1.abs();
    assert_eq!(abs1.get(), abs2.get());
}

#[test]
fn test_signum_type_precision() {
    // Test that signum provides the most precise type
    const POS: PositiveF64 = PositiveF64::new_const(5.0);
    const NEG: NegativeF64 = NegativeF64::new_const(-5.0);
    const FIN: FinF64 = FinF64::new_const(5.0);

    // Positive → Normalized (more precise than Symmetric)
    let sign_pos: NormalizedF64 = POS.signum();

    // Negative → NegativeNormalized (more precise than Symmetric)
    let sign_neg: NegativeNormalizedF64 = NEG.signum();

    // Fin → Symmetric (most general)
    let sign_fin: SymmetricF64 = FIN.signum();

    assert_eq!(sign_pos.get(), 1.0);
    assert_eq!(sign_neg.get(), -1.0);
    assert_eq!(sign_fin.get(), 1.0);
}

// ============================================================================
// sin() Operation Tests
// ============================================================================

#[test]
fn test_sin_basic() {
    let angle = FinF64::new(std::f64::consts::PI / 2.0).unwrap();
    let result: SymmetricF64 = angle.sin();
    assert_eq!(result.get(), 1.0);
}

#[test]
fn test_sin_zero() {
    let zero = FinF64::new(0.0).unwrap();
    let result: SymmetricF64 = zero.sin();
    assert_eq!(result.get(), 0.0);
}

#[test]
fn test_sin_pi() {
    let pi = FinF64::new(std::f64::consts::PI).unwrap();
    let result: SymmetricF64 = pi.sin();
    assert!((result.get() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_sin_various_types() {
    let fin = FinF64::new(std::f64::consts::PI / 6.0).unwrap();
    let pos = PositiveF64::new_const(0.0);
    let sym = SymmetricF64::new_const(0.5);

    let sin_fin: SymmetricF64 = fin.sin();
    let sin_pos: SymmetricF64 = pos.sin();
    let sin_sym: SymmetricF64 = sym.sin();

    assert!((sin_fin.get() - 0.5).abs() < f64::EPSILON);
    assert_eq!(sin_pos.get(), 0.0);
    assert!((sin_sym.get() - 0.479425538604203).abs() < f64::EPSILON);
}

// ============================================================================
// cos() Operation Tests
// ============================================================================

#[test]
fn test_cos_basic() {
    let angle = FinF64::new(0.0).unwrap();
    let result: SymmetricF64 = angle.cos();
    assert_eq!(result.get(), 1.0);
}

#[test]
fn test_cos_pi_half() {
    let pi_half = FinF64::new(std::f64::consts::PI / 2.0).unwrap();
    let result: SymmetricF64 = pi_half.cos();
    assert!((result.get() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_cos_pi() {
    let pi = FinF64::new(std::f64::consts::PI).unwrap();
    let result: SymmetricF64 = pi.cos();
    assert!((result.get() - (-1.0)).abs() < f64::EPSILON);
}

#[test]
fn test_cos_various_types() {
    let fin = FinF64::new(std::f64::consts::PI / 3.0).unwrap();
    let pos = PositiveF64::new_const(0.0);

    let cos_fin: SymmetricF64 = fin.cos();
    let cos_pos: SymmetricF64 = pos.cos();

    assert!((cos_fin.get() - 0.5).abs() < f64::EPSILON);
    assert_eq!(cos_pos.get(), 1.0);
}

// ============================================================================
// tan() Operation Tests
// ============================================================================

#[test]
fn test_tan_basic() {
    let angle = FinF64::new(0.0).unwrap();
    let result: Result<FinF64, FloatError> = angle.tan();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 0.0);
}

#[test]
fn test_tan_pi_four() {
    let pi_over_4 = FinF64::new(std::f64::consts::PI / 4.0).unwrap();
    let result: Result<FinF64, FloatError> = pi_over_4.tan();
    assert!(result.is_ok());
    assert!((result.unwrap().get() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_tan_various_types() {
    let fin = FinF64::new(std::f64::consts::PI / 6.0).unwrap();
    let pos = PositiveF64::new_const(0.0);

    let tan_fin: Result<FinF64, FloatError> = fin.tan();
    let tan_pos: Result<FinF64, FloatError> = pos.tan();

    assert!(tan_fin.is_ok());
    assert!(tan_pos.is_ok());

    // tan(π/6) = 1/√3 ≈ 0.5773502691896257
    let expected = 0.5773502691896257_f64;
    assert!((tan_fin.unwrap().get() - expected).abs() < f64::EPSILON);
    assert_eq!(tan_pos.unwrap().get(), 0.0);
}

#[test]
fn test_tan_singular_point() {
    // tan 在奇异点附近会产生非常大的值
    let close_to_singular = FinF64::new((std::f64::consts::PI / 2.0) - 0.0001).unwrap();
    let result: Result<FinF64, FloatError> = close_to_singular.tan();
    // 非常接近奇异点，应该返回一个很大的有限值
    assert!(result.is_ok());
    assert!(result.unwrap().get().abs() > 1000.0);

    // 使用一个值使得 tan 结果非常大但不是无穷大
    let very_close = FinF64::new((std::f64::consts::PI / 2.0) - 1e-10).unwrap();
    let result2: Result<FinF64, FloatError> = very_close.tan();
    // 这可能产生无穷大，也可能产生非常大的有限值
    // 我们只检查它不会 panic
    let _ = result2;
}

// ============================================================================
// Combined Tests
// ============================================================================

#[test]
fn test_trig_identities() {
    let angle = FinF64::new(std::f64::consts::PI / 4.0).unwrap();

    let sin_val: SymmetricF64 = angle.sin();
    let cos_val: SymmetricF64 = angle.cos();

    // sin²(x) + cos²(x) = 1
    let sum_of_squares = sin_val.get() * sin_val.get() + cos_val.get() * cos_val.get();
    assert!((sum_of_squares - 1.0).abs() < f64::EPSILON * 10.0);
}

#[test]
fn test_trig_with_negation() {
    let angle = FinF64::new(std::f64::consts::PI / 6.0).unwrap();

    let sin_pos: SymmetricF64 = angle.sin();
    let sin_neg: SymmetricF64 = (-angle).sin();

    // sin(-x) = -sin(x)
    assert!((sin_pos.get() + sin_neg.get()).abs() < f64::EPSILON);

    let cos_pos: SymmetricF64 = angle.cos();
    let cos_neg: SymmetricF64 = (-angle).cos();

    // cos(-x) = cos(x)
    assert!((cos_pos.get() - cos_neg.get()).abs() < f64::EPSILON);
}
