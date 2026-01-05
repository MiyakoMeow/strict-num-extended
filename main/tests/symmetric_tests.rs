//! # Symmetric 类型测试
//!
//! 测试值范围 [-1.0, 1.0] 的 Symmetric 类型的所有功能

// Strict floating-point comparisons and unwrap usage in test code are justified
#![expect(clippy::float_cmp, clippy::unwrap_used)]

use strict_num_extended::*;

/// `SymmetricF32` 基础功能测试
mod test_symmetric_f32_basic {
    use super::*;

    #[test]
    fn test_symmetric_f32_new_valid() {
        assert!(SymmetricF32::new(1.0).is_some());
        assert!(SymmetricF32::new(-1.0).is_some());
        assert!(SymmetricF32::new(0.0).is_some());
        assert!(SymmetricF32::new(0.75).is_some());
        assert!(SymmetricF32::new(-0.5).is_some());
    }

    #[test]
    fn test_symmetric_f32_new_invalid() {
        // 超出范围
        assert!(SymmetricF32::new(1.1).is_none());
        assert!(SymmetricF32::new(-1.1).is_none());

        // 特殊值
        assert!(SymmetricF32::new(f32::NAN).is_none());
        assert!(SymmetricF32::new(f32::INFINITY).is_none());
        assert!(SymmetricF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_symmetric_f32_get() {
        let val = SymmetricF32::new(0.75).unwrap();
        assert!((val.get() - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn test_symmetric_f32_debug() {
        let val = SymmetricF32::new(0.5).unwrap();
        let debug_str = format!("{:?}", val);
        // Debug 格式包含值信息即可
        assert!(debug_str.contains("0.5"));
    }
}

/// `SymmetricF64` 基础功能测试
mod test_symmetric_f64_basic {
    use super::*;

    #[test]
    fn test_symmetric_f64_new_valid() {
        assert!(SymmetricF64::new(1.0).is_some());
        assert!(SymmetricF64::new(-1.0).is_some());
        assert!(SymmetricF64::new(0.0).is_some());
        assert!(SymmetricF64::new(0.5).is_some());
        assert!(SymmetricF64::new(-0.25).is_some());
    }

    #[test]
    fn test_symmetric_f64_new_invalid() {
        // 超出范围
        assert!(SymmetricF64::new(1.001).is_none());
        assert!(SymmetricF64::new(-1.001).is_none());

        // 特殊值
        assert!(SymmetricF64::new(f64::NAN).is_none());
        assert!(SymmetricF64::new(f64::INFINITY).is_none());
        assert!(SymmetricF64::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_symmetric_f64_get() {
        let val = SymmetricF64::new(-0.5).unwrap();
        assert!((val.get() - (-0.5)).abs() < f64::EPSILON);
    }
}

/// 测试边界值
mod test_symmetric_boundaries {
    use super::*;

    #[test]
    fn test_symmetric_boundary_values_f32() {
        // 最小边界
        let min = SymmetricF32::new(-1.0).unwrap();
        assert_eq!(min.get(), -1.0);

        // 最大边界
        let max = SymmetricF32::new(1.0).unwrap();
        assert_eq!(max.get(), 1.0);

        // 零值
        let zero = SymmetricF32::new(0.0).unwrap();
        assert_eq!(zero.get(), 0.0);
    }

    #[test]
    fn test_symmetric_boundary_values_f64() {
        // 最小边界
        let min = SymmetricF64::new(-1.0).unwrap();
        assert_eq!(min.get(), -1.0);

        // 最大边界
        let max = SymmetricF64::new(1.0).unwrap();
        assert_eq!(max.get(), 1.0);

        // 零值
        let zero = SymmetricF64::new(0.0).unwrap();
        assert_eq!(zero.get(), 0.0);
    }

    #[test]
    fn test_symmetric_just_outside_boundaries() {
        // 超出边界一点应该失败
        assert!(SymmetricF32::new(-1.00001).is_none());
        assert!(SymmetricF32::new(1.00001).is_none());
        assert!(SymmetricF64::new(-1.00001).is_none());
        assert!(SymmetricF64::new(1.00001).is_none());
    }
}

/// 测试算术运算
mod test_symmetric_arithmetic {
    use super::*;

    #[test]
    fn test_symmetric_f32_addition() {
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(0.3).unwrap();
        let sum = a + b;
        assert_eq!(sum.get(), 0.8);
    }

    #[test]
    fn test_symmetric_f64_addition() {
        let a = SymmetricF64::new(-0.5).unwrap();
        let b = SymmetricF64::new(0.3).unwrap();
        let sum = a + b;
        assert_eq!(sum.get(), -0.2);
    }

    #[test]
    fn test_symmetric_f32_subtraction() {
        let a = SymmetricF32::new(0.8).unwrap();
        let b = SymmetricF32::new(0.3).unwrap();
        let diff = a - b;
        assert_eq!(diff.get(), 0.5);
    }

    #[test]
    fn test_symmetric_f64_subtraction() {
        let a = SymmetricF64::new(-0.2).unwrap();
        let b = SymmetricF64::new(0.3).unwrap();
        let diff = a - b;
        assert_eq!(diff.get(), -0.5);
    }

    #[test]
    fn test_symmetric_f32_multiplication() {
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(0.4).unwrap();
        let product = a * b;
        assert_eq!(product.get(), 0.2);
    }

    #[test]
    fn test_symmetric_f64_multiplication() {
        let a = SymmetricF64::new(-0.5).unwrap();
        let b = SymmetricF64::new(0.6).unwrap();
        let product = a * b;
        assert_eq!(product.get(), -0.3);
    }

    #[test]
    fn test_symmetric_f32_division() {
        // 使用在范围内结果进行测试
        // 除法结果必须在 [-1, 1] 范围内
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(1.0).unwrap();
        let quotient = a / b;
        assert_eq!(quotient.get(), 0.5);
    }

    #[test]
    fn test_symmetric_f64_division() {
        // 使用在范围内结果进行测试
        let a = SymmetricF64::new(-0.5).unwrap();
        let b = SymmetricF64::new(1.0).unwrap();
        let quotient = a / b;
        assert_eq!(quotient.get(), -0.5);
    }

    #[test]
    fn test_symmetric_arithmetic_panic_on_overflow() {
        // 算术运算结果超出范围应该 panic
        let a = SymmetricF32::new(0.8).unwrap();
        let b = SymmetricF32::new(0.5).unwrap();
        // 这个运算结果为 1.3，超出了 Symmetric 的范围，应该 panic
        let result = std::panic::catch_unwind(|| {
            let _ = a + b;
        });
        assert!(result.is_err());
    }
}

/// 测试比较操作
mod test_symmetric_comparison {
    use super::*;

    #[test]
    fn test_symmetric_f32_comparison() {
        let a = SymmetricF32::new(-0.5).unwrap();
        let b = SymmetricF32::new(0.5).unwrap();
        let c = SymmetricF32::new(0.5).unwrap();

        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
        assert_eq!(b, c);
        assert_ne!(a, b);
    }

    #[test]
    fn test_symmetric_f64_comparison() {
        let a = SymmetricF64::new(-1.0).unwrap();
        let b = SymmetricF64::new(0.0).unwrap();
        let c = SymmetricF64::new(1.0).unwrap();

        assert!(a < b);
        assert!(b < c);
        assert!(a <= b);
        assert!(b <= c);
        assert!(c > a);
        assert!(c >= b);
    }

    #[test]
    fn test_symmetric_equality() {
        let a = SymmetricF32::new(0.75).unwrap();
        let b = SymmetricF32::new(0.75).unwrap();
        assert_eq!(a, b);

        let c = SymmetricF64::new(-0.5).unwrap();
        let d = SymmetricF64::new(-0.5).unwrap();
        assert_eq!(c, d);
    }
}

/// 测试取负操作（自反性）
mod test_symmetric_negation {
    use super::*;

    #[test]
    fn test_symmetric_f32_negation() {
        let val = SymmetricF32::new(0.75).unwrap();
        let neg_val: SymmetricF32 = -val;
        assert_eq!(neg_val.get(), -0.75);
    }

    #[test]
    fn test_symmetric_f64_negation() {
        let val = SymmetricF64::new(-0.5).unwrap();
        let neg_val: SymmetricF64 = -val;
        assert_eq!(neg_val.get(), 0.5);
    }

    #[test]
    fn test_symmetric_double_negation_f32() {
        let original = SymmetricF32::new(0.75).unwrap();
        let neg1: SymmetricF32 = -original;
        let neg2: SymmetricF32 = -neg1;
        assert_eq!(neg2.get(), 0.75);
    }

    #[test]
    fn test_symmetric_double_negation_f64() {
        let original = SymmetricF64::new(-0.5).unwrap();
        let neg1: SymmetricF64 = -original;
        let neg2: SymmetricF64 = -neg1;
        assert_eq!(neg2.get(), -0.5);
    }

    #[test]
    fn test_symmetric_negation_boundary_values() {
        // 测试边界值的取负
        let max = SymmetricF32::new(1.0).unwrap();
        let neg_max: SymmetricF32 = -max;
        assert_eq!(neg_max.get(), -1.0);

        let min = SymmetricF32::new(-1.0).unwrap();
        let neg_min: SymmetricF32 = -min;
        assert_eq!(neg_min.get(), 1.0);

        let zero = SymmetricF32::new(0.0).unwrap();
        let neg_zero: SymmetricF32 = -zero;
        assert_eq!(neg_zero.get(), -0.0);
    }

    #[test]
    fn test_symmetric_negation_reflexive_property() {
        // 验证 Symmetric 的取负操作是自反的
        let val = SymmetricF64::new(0.123).unwrap();
        let neg_val: SymmetricF64 = -val;
        // 类型应该仍然是 SymmetricF64，不是其他类型
        let _check: SymmetricF64 = neg_val;
    }
}

/// 测试编译时常量
mod test_symmetric_new_const {
    use super::*;

    #[test]
    fn test_symmetric_f32_new_const() {
        const VAL: SymmetricF32 = SymmetricF32::new_const(0.5);
        assert_eq!(VAL.get(), 0.5);
    }

    #[test]
    fn test_symmetric_f64_new_const() {
        const VAL: SymmetricF64 = SymmetricF64::new_const(-0.75);
        assert_eq!(VAL.get(), -0.75);
    }

    #[test]
    fn test_symmetric_f32_boundary_const() {
        const MIN: SymmetricF32 = SymmetricF32::new_const(-1.0);
        const MAX: SymmetricF32 = SymmetricF32::new_const(1.0);
        const ZERO: SymmetricF32 = SymmetricF32::new_const(0.0);

        assert_eq!(MIN.get(), -1.0);
        assert_eq!(MAX.get(), 1.0);
        assert_eq!(ZERO.get(), 0.0);
    }

    #[test]
    fn test_symmetric_f64_boundary_const() {
        const MIN: SymmetricF64 = SymmetricF64::new_const(-1.0);
        const MAX: SymmetricF64 = SymmetricF64::new_const(1.0);
        const ZERO: SymmetricF64 = SymmetricF64::new_const(0.0);

        assert_eq!(MIN.get(), -1.0);
        assert_eq!(MAX.get(), 1.0);
        assert_eq!(ZERO.get(), 0.0);
    }
}

/// 测试 Option 类型
mod test_symmetric_option {
    use super::*;

    #[test]
    fn test_opt_symmetric_f32_basic() {
        let opt_val: OptSymmetricF32 = Some(SymmetricF32::new(0.5).unwrap());
        assert!(opt_val.is_some());

        let opt_none: OptSymmetricF32 = None;
        assert!(opt_none.is_none());
    }

    #[test]
    fn test_opt_symmetric_f64_basic() {
        let opt_val: OptSymmetricF64 = Some(SymmetricF64::new(-0.5).unwrap());
        assert!(opt_val.is_some());

        let opt_none: OptSymmetricF64 = None;
        assert!(opt_none.is_none());
    }

    #[test]
    fn test_opt_symmetric_option_unwrap() {
        let a = SymmetricF32::new(0.5).unwrap();
        let b = SymmetricF32::new(0.3).unwrap();

        // 直接对值进行算术运算
        let sum = a + b;
        assert_eq!(sum.get(), 0.8);
    }

    #[test]
    fn test_opt_symmetric_option_with_none() {
        let a: OptSymmetricF32 = Some(SymmetricF32::new(0.5).unwrap());
        let b: OptSymmetricF32 = None;

        // 当有 None 时，算术运算不会执行
        assert!(a.is_some());
        assert!(b.is_none());
    }
}

/// 测试类型转换
mod test_symmetric_conversion {
    use super::*;

    #[test]
    fn test_symmetric_try_from_valid_f32() {
        // 从基本 f32 类型转换（有效值）
        let value: f32 = 0.5;
        let sym = SymmetricF32::try_from(value).unwrap();
        assert_eq!(sym.get(), 0.5);
    }

    #[test]
    fn test_symmetric_try_from_valid_f64() {
        // 从基本 f64 类型转换（有效值）
        let value: f64 = -0.5;
        let sym = SymmetricF64::try_from(value).unwrap();
        assert_eq!(sym.get(), -0.5);
    }

    #[test]
    fn test_symmetric_try_from_invalid_f32() {
        // 从基本 f32 类型转换（超出范围）
        let value: f32 = 2.0;
        let result = SymmetricF32::try_from(value);
        assert!(result.is_err());
    }

    #[test]
    fn test_symmetric_try_from_invalid_f64() {
        // 从基本 f64 类型转换（超出范围）
        let value: f64 = 1.5;
        let result = SymmetricF64::try_from(value);
        assert!(result.is_err());
    }

    #[test]
    fn test_symmetric_from_fin_type() {
        // FinF32 是更宽松的类型，包含所有有限值
        // 可以通过 get() 获取值，然后重新包装为 SymmetricF32
        let fin = FinF32::new(0.5).unwrap();
        let sym = SymmetricF32::new(fin.get()).unwrap();
        assert_eq!(sym.get(), 0.5);

        // 超出范围的值无法转换
        let fin_invalid = FinF32::new(2.0).unwrap();
        let sym_invalid = SymmetricF32::new(fin_invalid.get());
        assert!(sym_invalid.is_none());
    }

    #[test]
    fn test_symmetric_from_normalized_types() {
        // NormalizedF32 的范围 [0, 1] 是 Symmetric 的子集
        let norm = NormalizedF32::new(0.5).unwrap();
        let sym = SymmetricF32::new(norm.get()).unwrap();
        assert_eq!(sym.get(), 0.5);

        // NegativeNormalizedF32 的范围 [-1, 0] 也是 Symmetric 的子集
        let neg_norm = NegativeNormalizedF32::new(-0.5).unwrap();
        let sym2 = SymmetricF32::new(neg_norm.get()).unwrap();
        assert_eq!(sym2.get(), -0.5);
    }
}
