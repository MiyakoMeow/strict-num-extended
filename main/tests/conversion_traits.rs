//! From/TryFrom trait 测试

use std::convert::TryFrom;

// 1. 约束类型 → 原语
#[test]
fn test_constraint_to_primitive() {
    let fin = strict_num_extended::FinF32::new_const(3.14);
    let f32_val: f32 = fin.into();
    assert_eq!(f32_val, 3.14);
}

// 2. 原语 → 约束类型 (成功)
#[test]
fn test_primitive_to_constraint_success() {
    let fin: strict_num_extended::FinF32 = strict_num_extended::FinF32::try_from(3.14).unwrap();
    assert_eq!(fin.get(), 3.14);
}

// 3. 原语 → 约束类型 (失败)
#[test]
fn test_primitive_to_constraint_failure() {
    let result: Result<strict_num_extended::FinF32, _> =
        strict_num_extended::FinF32::try_from(f32::NAN);
    assert!(result.is_err());
}

// 4. 子集 → 超集 (From)
#[test]
fn test_subset_to_superset() {
    let normalized = strict_num_extended::NormalizedF32::new_const(0.5);
    let fin: strict_num_extended::FinF32 = normalized.into();
    assert_eq!(fin.get(), 0.5);
}

// 5. 超集 → 子集 (TryFrom 成功)
#[test]
fn test_superset_to_subset_success() {
    let fin = strict_num_extended::FinF32::new_const(0.5);
    let normalized: Result<strict_num_extended::NormalizedF32, _> = fin.try_into();
    assert!(normalized.is_ok());
}

// 6. 超集 → 子集 (TryFrom 失败)
#[test]
fn test_superset_to_subset_failure() {
    let fin = strict_num_extended::FinF32::new_const(2.0);
    let normalized: Result<strict_num_extended::NormalizedF32, _> = fin.try_into();
    assert!(normalized.is_err());
}

// 7. F32 → F64 (From)
#[test]
fn test_f32_to_f64_from() {
    let fin32 = strict_num_extended::FinF32::new_const(3.0);
    let fin64: strict_num_extended::FinF64 = fin32.into();
    assert_eq!(fin64.get(), 3.0);
}

// 8. F64 → F32 (TryFrom)
#[test]
fn test_f64_to_f32_tryfrom() {
    let fin64 = strict_num_extended::FinF64::new_const(3.0);
    let fin32: Result<strict_num_extended::FinF32, _> = fin64.try_into();
    assert!(fin32.is_ok());
}
