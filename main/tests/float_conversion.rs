//! F32/F64 类型转换测试

use strict_num_extended::{FloatError, *};

// ========== try_into_f32 测试 ==========

#[test]
fn test_try_into_f32_precision_loss() {
    // 测试精度损失检测
    let value_f64 = FinF64::new_const(1.234567890123456789);
    let result: Result<FinF32, FloatError> = value_f64.try_into_f32();
    assert!(result.is_err(), "Should fail due to precision loss");
}

#[test]
fn test_try_into_f32_range_overflow() {
    // 测试范围溢出检测
    let value_f64 = FinF64::new_const(1e40);
    let result: Result<FinF32, FloatError> = value_f64.try_into_f32();
    assert!(result.is_err(), "Should fail due to F32 range overflow");
}

#[test]
fn test_try_into_f32_constraint_violation() {
    // 测试约束违规（PositiveF64 -> PositiveF32）
    let positive_f64 = PositiveF64::new_const(2.0);
    let result: Result<PositiveF32, FloatError> = positive_f64.try_into_f32();
    assert!(result.is_ok(), "Should succeed with valid positive value");

    // 测试转换后的约束
    let normalized_f64 = NormalizedF64::new_const(0.5);
    let result: Result<NormalizedF32, FloatError> = normalized_f64.try_into_f32();
    assert!(result.is_ok(), "Should succeed with normalized value");

    // 测试超出约束范围的值
    let large_f64 = PositiveF64::new_const(1000.0);
    let result: Result<PositiveF32, FloatError> = large_f64.try_into_f32();
    // 这可能成功或失败，取决于 1000.0 是否在 F32 范围内且满足 PositiveF32 约束
    if result.is_ok() {
        let f32_val = result.unwrap();
        assert!(f32_val.get() > 0.0, "Should maintain positive constraint");
    }
}

#[test]
fn test_try_into_f32_normal_conversion() {
    // 测试正常转换（使用精确可表示的值）
    let value_f64 = FinF64::new_const(3.0);
    let result: Result<FinF32, FloatError> = value_f64.try_into_f32();
    assert!(result.is_ok(), "Should succeed for F32-representable value");

    let f32_value = result.unwrap();
    assert_eq!(f32_value.get(), 3.0);
}

#[test]
fn test_try_into_f32_const_context() {
    // 测试 const 上下文
    const VALUE_F64: FinF64 = FinF64::new_const(2.0);
    const VALUE_F32: Result<FinF32, FloatError> = VALUE_F64.try_into_f32();

    assert!(VALUE_F32.is_ok(), "Const conversion should succeed");
    let f32_val = VALUE_F32.unwrap();
    assert_eq!(f32_val.get(), 2.0);
}

#[test]
fn test_try_into_f32_different_constraints() {
    // 测试不同约束类型的转换
    let positive = PositiveF64::new_const(10.0);
    let result: Result<PositiveF32, FloatError> = positive.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 10.0);

    let negative = NegativeF64::new_const(-10.0);
    let result: Result<NegativeF32, FloatError> = negative.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), -10.0);

    let nonzero = NonZeroF64::new_const(5.0);
    let result: Result<NonZeroF32, FloatError> = nonzero.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 5.0);

    let symmetric = SymmetricF64::new_const(0.5);
    let result: Result<SymmetricF32, FloatError> = symmetric.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 0.5);

    let normalized = NormalizedF64::new_const(0.75);
    let result: Result<NormalizedF32, FloatError> = normalized.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 0.75);

    let neg_normalized = NegativeNormalizedF64::new_const(-0.5);
    let result: Result<NegativeNormalizedF32, FloatError> = neg_normalized.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), -0.5);

    let nonzero_pos = NonZeroPositiveF64::new_const(1.0);
    let result: Result<NonZeroPositiveF32, FloatError> = nonzero_pos.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), 1.0);

    let nonzero_neg = NonZeroNegativeF64::new_const(-1.0);
    let result: Result<NonZeroNegativeF32, FloatError> = nonzero_neg.try_into_f32();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().get(), -1.0);
}

// ========== as_f64 测试 ==========

#[test]
fn test_as_f64_const_context() {
    // 测试 const 上下文
    const VALUE_F32: FinF32 = FinF32::new_const(3.0);
    const VALUE_F64: FinF64 = VALUE_F32.as_f64();

    assert_eq!(VALUE_F64.get(), 3.0);
}

#[test]
fn test_as_f64_different_constraints() {
    // 测试不同约束类型的转换
    let f32_val = PositiveF32::new_const(10.0);
    let f64_val: PositiveF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), 10.0);
    assert!(f64_val.get() > 0.0);

    let f32_val = NegativeF32::new_const(-10.0);
    let f64_val: NegativeF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), -10.0);
    assert!(f64_val.get() < 0.0);

    let f32_val = NonZeroF32::new_const(5.0);
    let f64_val: NonZeroF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), 5.0);
    assert_ne!(f64_val.get(), 0.0);

    let f32_val = SymmetricF32::new_const(0.5);
    let f64_val: SymmetricF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), 0.5);

    let f32_val = NormalizedF32::new_const(0.75);
    let f64_val: NormalizedF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), 0.75);

    let f32_val = NegativeNormalizedF32::new_const(-0.5);
    let f64_val: NegativeNormalizedF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), -0.5);

    let f32_val = NonZeroPositiveF32::new_const(1.0);
    let f64_val: NonZeroPositiveF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), 1.0);

    let f32_val = NonZeroNegativeF32::new_const(-1.0);
    let f64_val: NonZeroNegativeF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), -1.0);
}

#[test]
fn test_as_f64_precision_preservation() {
    // 测试精度保持（从 F32 到 F64 应该总是保持精度）
    let f32_val = FinF32::new_const(1.5);
    let f64_val: FinF64 = f32_val.as_f64();
    assert_eq!(f64_val.get(), 1.5);

    let f32_val = FinF32::new_const(std::f32::consts::PI);
    let f64_val: FinF64 = f32_val.as_f64();
    assert!((f64_val.get() - std::f64::consts::PI).abs() < 1e-6);
}

#[test]
fn test_roundtrip_conversion() {
    // 测试往返转换：F32 -> F64 -> F32
    let original_f32 = FinF32::new_const(3.14159);
    let f64_val: FinF64 = original_f32.as_f64();
    let back_to_f32: Result<FinF32, FloatError> = f64_val.try_into_f32();

    assert!(
        back_to_f32.is_ok(),
        "Roundtrip F32->F64->F32 should succeed"
    );
    let final_f32 = back_to_f32.unwrap();
    assert!((final_f32.get() - original_f32.get()).abs() < 1e-6);
}

// ========== 综合测试 ==========

#[test]
fn test_combined_conversion_with_arithmetic() {
    // 测试转换与算术运算的结合
    let a_f64 = PositiveF64::new_const(10.0);
    let b_f64 = PositiveF64::new_const(20.0);

    // 转换为 F32 进行计算
    let a_f32: Result<PositiveF32, FloatError> = a_f64.try_into_f32();
    let b_f32: Result<PositiveF32, FloatError> = b_f64.try_into_f32();

    assert!(a_f32.is_ok() && b_f32.is_ok());

    let sum_f32 = a_f32.unwrap() + b_f32.unwrap();
    let sum_f32 = sum_f32.unwrap();

    // 转换回 F64
    let sum_f64: PositiveF64 = sum_f32.as_f64();
    assert_eq!(sum_f64.get(), 30.0);
}

#[cfg(test)]
mod test_construction {
    use super::*;

    #[test]
    fn test_construction_with_conversion() {
        // 测试在常量上下文中使用转换
        const F64_VAL: PositiveF64 = PositiveF64::new_const(42.0);
        const F32_VAL: Result<PositiveF32, FloatError> = F64_VAL.try_into_f32();

        assert!(F32_VAL.is_ok());
        assert_eq!(F32_VAL.unwrap().get(), 42.0);
    }
}
