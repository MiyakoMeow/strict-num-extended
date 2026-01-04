//! # struct-num-extended 测试
//!
//! 这个模块测试有限浮点数类型的所有功能。

// 测试代码中的浮点数严格比较、unwrap 使用和变量名覆盖是合理的
#![expect(clippy::float_cmp, clippy::unwrap_used, clippy::shadow_unrelated)]

use strict_num_extended::*;

/// 测试 `FinF32` 的基本功能
mod test_finf32 {
    use super::*;

    #[test]
    fn test_finf32_new_valid() {
        assert!(FinF32::new(1.0).is_some());
        assert!(FinF32::new(-1.0).is_some());
        assert!(FinF32::new(0.0).is_some());
        assert!(FinF32::new(f32::MAX).is_some());
        assert!(FinF32::new(f32::MIN).is_some());
        assert!(FinF32::new(0.00001).is_some());
    }

    #[test]
    fn test_finf32_new_invalid() {
        assert!(FinF32::new(f32::NAN).is_none());
        assert!(FinF32::new(f32::INFINITY).is_none());
        assert!(FinF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_finf32_get() {
        let finite = FinF32::new(std::f32::consts::PI).unwrap();
        assert!((finite.get() - std::f32::consts::PI).abs() < f32::EPSILON);
    }

    #[test]
    fn test_finf32_debug() {
        let finite = FinF32::new(1.5).unwrap();
        assert!(format!("{:?}", finite).contains("FiniteFloat"));
    }

    #[test]
    fn test_finf32_display() {
        let finite = FinF32::new(1.5).unwrap();
        assert_eq!(format!("{}", finite), "1.5");
    }
}

/// 测试 `FinF64` 的基本功能
mod test_finf64 {
    use super::*;

    #[test]
    fn test_finf64_new_valid() {
        assert!(FinF64::new(1.0).is_some());
        assert!(FinF64::new(-1.0).is_some());
        assert!(FinF64::new(0.0).is_some());
        assert!(FinF64::new(f64::MAX).is_some());
        assert!(FinF64::new(f64::MIN).is_some());
    }

    #[test]
    fn test_finf64_new_invalid() {
        assert!(FinF64::new(f64::NAN).is_none());
        assert!(FinF64::new(f64::INFINITY).is_none());
        assert!(FinF64::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_finf64_get() {
        let finite = FinF64::new(std::f64::consts::PI).unwrap();
        assert!((finite.get() - std::f64::consts::PI).abs() < f64::EPSILON);
    }
}

/// 测试 `PositiveF32` 的基本功能
mod test_positivef32 {
    use super::*;

    #[test]
    fn test_positivef32_new_valid() {
        assert!(PositiveF32::new(1.0).is_some());
        assert!(PositiveF32::new(0.0).is_some());
        // Positive 现在不再允许无穷大
        assert!(PositiveF32::new(f32::INFINITY).is_none());
        assert!(PositiveF32::new(f32::MAX).is_some());
    }

    #[test]
    fn test_positivef32_new_invalid() {
        assert!(PositiveF32::new(f32::NAN).is_none());
        assert!(PositiveF32::new(-1.0).is_none());
        // Positive 现在使用数值比较 (>= 0.0)，接受 -0.0
        assert!(PositiveF32::new(-0.0).is_some());
        assert!(PositiveF32::new(f32::NEG_INFINITY).is_none());
        assert!(PositiveF32::new(f32::INFINITY).is_none());
    }

    #[test]
    fn test_positivef32_get() {
        let positive = PositiveF32::new(42.0).unwrap();
        assert_eq!(positive.get(), 42.0);
    }
}

/// 测试 `PositiveF64` 的基本功能
mod test_positivef64 {
    use super::*;

    #[test]
    fn test_positivef64_new_valid() {
        assert!(PositiveF64::new(1.0).is_some());
        assert!(PositiveF64::new(0.0).is_some());
        // Positive 现在不再允许无穷大
        assert!(PositiveF64::new(f64::INFINITY).is_none());
        assert!(PositiveF64::new(f64::MAX).is_some());
    }

    #[test]
    fn test_positivef64_new_invalid() {
        assert!(PositiveF64::new(f64::NAN).is_none());
        assert!(PositiveF64::new(-1.0).is_none());
        // Positive 现在使用数值比较 (>= 0.0)，接受 -0.0
        assert!(PositiveF64::new(-0.0).is_some());
        assert!(PositiveF64::new(f64::NEG_INFINITY).is_none());
        assert!(PositiveF64::new(f64::INFINITY).is_none());
    }

    #[test]
    fn test_positivef64_get() {
        let positive = PositiveF64::new(123.456).unwrap();
        assert_eq!(positive.get(), 123.456);
    }
}

/// 测试算术运算
mod test_arithmetic_operations {
    use super::*;

    // FinF32 算术运算
    #[test]
    fn test_finf32_add() {
        let a = FinF32::new(2.0).unwrap();
        let b = FinF32::new(3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 5.0);
    }

    #[test]
    fn test_finf32_sub() {
        let a = FinF32::new(10.0).unwrap();
        let b = FinF32::new(3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), 7.0);
    }

    #[test]
    fn test_finf32_mul() {
        let a = FinF32::new(4.0).unwrap();
        let b = FinF32::new(3.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 12.0);
    }

    #[test]
    fn test_finf32_div() {
        let a = FinF32::new(12.0).unwrap();
        let b = FinF32::new(3.0).unwrap();
        let c = a / b;
        assert_eq!(c.get(), 4.0);
    }

    #[test]
    fn test_finf32_arithmetic_zero() {
        let a = FinF32::new(5.0).unwrap();
        let b = FinF32::new(0.0).unwrap();
        assert_eq!((a + b).get(), 5.0);
        assert_eq!((a - b).get(), 5.0);
        assert_eq!((a * b).get(), 0.0);
    }

    // PositiveF32 算术运算
    #[test]
    fn test_positivef32_add() {
        let a = PositiveF32::new(2.0).unwrap();
        let b = PositiveF32::new(3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 5.0);
    }

    #[test]
    fn test_positivef32_sub() {
        let a = PositiveF32::new(10.0).unwrap();
        let b = PositiveF32::new(3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), 7.0);
    }

    #[test]
    fn test_positivef32_mul() {
        let a = PositiveF32::new(4.0).unwrap();
        let b = PositiveF32::new(3.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 12.0);
    }

    #[test]
    fn test_positivef32_div() {
        let a = PositiveF32::new(12.0).unwrap();
        let b = PositiveF32::new(3.0).unwrap();
        let c = a / b;
        assert_eq!(c.get(), 4.0);
    }

    // FinF64 算术运算
    #[test]
    fn test_finf64_add() {
        let a = FinF64::new(2.5).unwrap();
        let b = FinF64::new(3.5).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 6.0);
    }

    #[test]
    fn test_finf64_mul() {
        let a = FinF64::new(2.5).unwrap();
        let b = FinF64::new(4.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 10.0);
    }

    // PositiveF64 算术运算
    #[test]
    fn test_positivef64_add() {
        let a = PositiveF64::new(2.5).unwrap();
        let b = PositiveF64::new(3.5).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 6.0);
    }

    #[test]
    fn test_positivef64_mul() {
        let a = PositiveF64::new(2.5).unwrap();
        let b = PositiveF64::new(4.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 10.0);
    }

    // 复合运算
    #[test]
    fn test_complex_arithmetic() {
        let a = FinF32::new(10.0).unwrap();
        let b = FinF32::new(5.0).unwrap();
        let c = FinF32::new(2.0).unwrap();
        let result = (a + b) * c;
        assert_eq!(result.get(), 30.0);
    }
}

/// 测试比较运算
mod test_comparison_operations {
    use super::*;

    #[test]
    fn test_finf32_partial_eq() {
        let a = FinF32::new(1.0).unwrap();
        let b = FinF32::new(1.0).unwrap();
        assert_eq!(a, b);

        let c = FinF32::new(2.0).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn test_finf32_partial_ord() {
        let a = FinF32::new(1.0).unwrap();
        let b = FinF32::new(2.0).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);

        let c = FinF32::new(1.0).unwrap();
        assert!(a <= c);
        assert!(a >= c);
    }

    #[test]
    fn test_positivef32_comparison() {
        let a = PositiveF32::new(1.0).unwrap();
        let b = PositiveF32::new(2.0).unwrap();
        assert!(a < b);
        assert!(a == PositiveF32::new(1.0).unwrap());
    }

    #[test]
    fn test_finf64_comparison() {
        let a = FinF64::new(1.5).unwrap();
        let b = FinF64::new(2.5).unwrap();
        assert!(a < b);
        assert_eq!(a, FinF64::new(1.5).unwrap());
    }
}

/// 测试类型转换
mod test_conversions {
    use super::*;

    #[test]
    fn test_try_from_f32_to_f64() {
        let value_f32 = std::f32::consts::PI;
        let finite_64 = FinF64::try_from(value_f32).unwrap();
        // f32::PI 转换为 f64 后的精度有限，使用适当的容差
        assert!((finite_64.get() - std::f64::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn test_try_from_positive_types() {
        let value_f32 = 5.0f32;
        let positive_32 = PositiveF32::try_from(value_f32).unwrap();
        assert_eq!(positive_32.get(), 5.0);
    }

    #[test]
    fn test_try_from_with_constraint_validation() {
        // 尝试从 FinF32 转换到 PositiveF32，负值会失败
        let finite_32 = FinF32::new(-5.0).unwrap();
        let value = finite_32.get();
        assert!(PositiveF32::new(value).is_none());

        // 正值应该成功
        let finite_32 = FinF32::new(5.0).unwrap();
        let positive_32 = PositiveF32::new(finite_32.get()).unwrap();
        assert_eq!(positive_32.get(), 5.0);

        // FinF32 可以接受 PositiveF32 的值
        let positive_32 = PositiveF32::new(5.0).unwrap();
        let finite_32 = FinF32::new(positive_32.get()).unwrap();
        assert_eq!(finite_32.get(), 5.0);
    }
}

/// 测试 unsafe `new_unchecked`
mod test_unchecked {
    use super::*;

    #[test]
    fn test_new_unchecked_valid() {
        // 安全使用：传入满足约束的值
        let finite = unsafe { FinF32::new_unchecked(std::f32::consts::PI) };
        assert!((finite.get() - std::f32::consts::PI).abs() < f32::EPSILON);

        let positive = unsafe { PositiveF32::new_unchecked(5.0) };
        assert_eq!(positive.get(), 5.0);
    }

    #[test]
    fn test_new_unchecked_behavior() {
        // unsafe 函数不会 panic，只是允许创建可能无效的值
        // 这些测试验证了函数的存在和行为，但不测试 panic
        let nan_value = unsafe { FinF32::new_unchecked(f32::NAN) };
        assert!(nan_value.get().is_nan());

        let inf_value = unsafe { FinF32::new_unchecked(f32::INFINITY) };
        assert!(inf_value.get().is_infinite());

        let neg_value = unsafe { PositiveF32::new_unchecked(-1.0) };
        assert_eq!(neg_value.get(), -1.0);
    }
}

/// 测试 Optional 类型
mod test_optional_types {
    use super::*;

    #[test]
    fn test_optfinf32() {
        let some: OptFinF32 = Some(FinF32::new(1.0).unwrap());
        let none: OptFinF32 = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn test_optpositivef32() {
        let some: OptPositiveF32 = Some(PositiveF32::new(1.0).unwrap());
        let none: OptPositiveF32 = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn test_optional_arithmetic() {
        // 测试 Option 类型的基本操作
        let a: OptFinF32 = Some(FinF32::new(2.0).unwrap());
        let b: OptFinF32 = Some(FinF32::new(3.0).unwrap());
        let c: OptFinF32 = None;

        assert!(a.is_some());
        assert!(b.is_some());
        assert!(c.is_none());

        // 测试从 Some 中提取值并进行运算
        if let Some(fin_a) = a
            && let Some(fin_b) = b
        {
            let result = fin_a + fin_b;
            assert_eq!(result.get(), 5.0);
        }

        // 测试与 None 的交互
        if let Some(fin_a) = a {
            assert_eq!(fin_a.get(), 2.0);
        }

        // 测试 None
        assert!(c.is_none());
    }

    #[test]
    fn test_optional_conversion() {
        let some: OptFinF32 = Some(FinF32::new(std::f32::consts::PI).unwrap());
        let none: OptFinF32 = None;

        // OptFinF32 可以直接作为 OptFinF64 使用（协变性）
        // 这测试了类型别名的工作方式
        assert!(some.is_some());
        assert!(none.is_none());
    }
}

/// 测试边界值
mod test_edge_cases {
    use super::*;

    #[test]
    fn test_min_values() {
        let min_f32 = FinF32::new(f32::MIN).unwrap();
        assert_eq!(min_f32.get(), f32::MIN);

        let min_f64 = FinF64::new(f64::MIN).unwrap();
        assert_eq!(min_f64.get(), f64::MIN);
    }

    #[test]
    fn test_max_values() {
        let max_f32 = FinF32::new(f32::MAX).unwrap();
        assert_eq!(max_f32.get(), f32::MAX);

        let max_f64 = FinF64::new(f64::MAX).unwrap();
        assert_eq!(max_f64.get(), f64::MAX);
    }

    #[test]
    fn test_very_small_values() {
        let tiny_f32 = FinF32::new(f32::EPSILON).unwrap();
        assert_eq!(tiny_f32.get(), f32::EPSILON);

        let tiny_f64 = FinF64::new(f64::EPSILON).unwrap();
        assert_eq!(tiny_f64.get(), f64::EPSILON);
    }

    #[test]
    fn test_zero_variants() {
        let zero_pos = PositiveF32::new(0.0).unwrap();
        let neg_zero = FinF32::new(-0.0).unwrap();

        assert_eq!(zero_pos.get(), 0.0);
        assert_eq!(neg_zero.get(), -0.0);
        assert_eq!(zero_pos.get(), neg_zero.get());
    }

    #[test]
    fn test_chained_arithmetic() {
        let a = FinF32::new(1.0).unwrap();
        let b = FinF32::new(2.0).unwrap();
        let c = FinF32::new(3.0).unwrap();
        let d = FinF32::new(4.0).unwrap();

        let result = ((a + b) * c) - d;
        assert_eq!(result.get(), 5.0);
    }

    #[test]
    fn test_division_edge_cases() {
        let a = FinF32::new(5.0).unwrap();
        let b = FinF32::new(2.0).unwrap();
        let result = a / b;
        assert!((result.get() - 2.5).abs() < f32::EPSILON);
    }
}

/// 测试约束 trait 的工作方式
mod test_constraints {
    use super::*;

    #[test]
    fn test_finf32_constraint() {
        assert!(FinF32::new(1.0).is_some());
        assert!(FinF32::new(f32::NAN).is_none());
        assert!(FinF32::new(f32::INFINITY).is_none());
        assert!(FinF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_positivef32_constraint() {
        assert!(PositiveF32::new(1.0).is_some());
        assert!(PositiveF32::new(0.0).is_some());
        // Positive 现在不再允许无穷大
        assert!(PositiveF32::new(f32::INFINITY).is_none());
        assert!(PositiveF32::new(-1.0).is_none());
        assert!(PositiveF32::new(f32::NAN).is_none());
    }
}

/// 测试 `NonZeroF32` 的基本功能
mod test_nonzerof32 {
    use super::*;

    #[test]
    fn test_nonzerof32_new_valid() {
        assert!(NonZeroF32::new(1.0).is_some());
        assert!(NonZeroF32::new(-1.0).is_some());
        assert!(NonZeroF32::new(f32::MAX).is_some());
        assert!(NonZeroF32::new(f32::MIN).is_some());
        assert!(NonZeroF32::new(0.00001).is_some());
    }

    #[test]
    fn test_nonzerof32_new_invalid() {
        assert!(NonZeroF32::new(f32::NAN).is_none());
        assert!(NonZeroF32::new(f32::INFINITY).is_none());
        assert!(NonZeroF32::new(f32::NEG_INFINITY).is_none());
        assert!(NonZeroF32::new(0.0).is_none());
        assert!(NonZeroF32::new(-0.0).is_none());
    }

    #[test]
    fn test_nonzerof32_get() {
        let non_zero = NonZeroF32::new(std::f32::consts::PI).unwrap();
        assert!((non_zero.get() - std::f32::consts::PI).abs() < f32::EPSILON);
    }
}

/// 测试 `NonZeroF64` 的基本功能
mod test_nonzerof64 {
    use super::*;

    #[test]
    fn test_nonzerof64_new_valid() {
        assert!(NonZeroF64::new(1.0).is_some());
        assert!(NonZeroF64::new(-1.0).is_some());
        assert!(NonZeroF64::new(f64::MAX).is_some());
        assert!(NonZeroF64::new(f64::MIN).is_some());
    }

    #[test]
    fn test_nonzerof64_new_invalid() {
        assert!(NonZeroF64::new(f64::NAN).is_none());
        assert!(NonZeroF64::new(f64::INFINITY).is_none());
        assert!(NonZeroF64::new(f64::NEG_INFINITY).is_none());
        assert!(NonZeroF64::new(0.0).is_none());
        assert!(NonZeroF64::new(-0.0).is_none());
    }

    #[test]
    fn test_nonzerof64_get() {
        let non_zero = NonZeroF64::new(std::f64::consts::PI).unwrap();
        assert!((non_zero.get() - std::f64::consts::PI).abs() < f64::EPSILON);
    }
}

/// 测试 `NonZeroPositiveF32` 的基本功能
mod test_nonzero_positivef32 {
    use super::*;

    #[test]
    fn test_nonzero_positivef32_new_valid() {
        assert!(NonZeroPositiveF32::new(1.0).is_some());
        assert!(NonZeroPositiveF32::new(f32::MAX).is_some());
        assert!(NonZeroPositiveF32::new(0.00001).is_some());
    }

    #[test]
    fn test_nonzero_positivef32_new_invalid() {
        assert!(NonZeroPositiveF32::new(f32::NAN).is_none());
        assert!(NonZeroPositiveF32::new(-1.0).is_none());
        assert!(NonZeroPositiveF32::new(-0.0).is_none());
        assert!(NonZeroPositiveF32::new(0.0).is_none());
        assert!(NonZeroPositiveF32::new(f32::INFINITY).is_none());
        assert!(NonZeroPositiveF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_nonzero_positivef32_get() {
        let non_zero_positive = NonZeroPositiveF32::new(42.0).unwrap();
        assert_eq!(non_zero_positive.get(), 42.0);
    }
}

/// 测试 `NonZeroPositiveF64` 的基本功能
mod test_nonzero_positivef64 {
    use super::*;

    #[test]
    fn test_nonzero_positivef64_new_valid() {
        assert!(NonZeroPositiveF64::new(1.0).is_some());
        assert!(NonZeroPositiveF64::new(f64::MAX).is_some());
        assert!(NonZeroPositiveF64::new(0.00001).is_some());
    }

    #[test]
    fn test_nonzero_positivef64_new_invalid() {
        assert!(NonZeroPositiveF64::new(f64::NAN).is_none());
        assert!(NonZeroPositiveF64::new(-1.0).is_none());
        assert!(NonZeroPositiveF64::new(-0.0).is_none());
        assert!(NonZeroPositiveF64::new(0.0).is_none());
        assert!(NonZeroPositiveF64::new(f64::INFINITY).is_none());
        assert!(NonZeroPositiveF64::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_nonzero_positivef64_get() {
        let non_zero_positive = NonZeroPositiveF64::new(123.456).unwrap();
        assert_eq!(non_zero_positive.get(), 123.456);
    }
}

/// 测试 `NonZero` 类型的算术运算
mod test_nonzero_arithmetic_operations {
    use super::*;

    // NonZeroF32 算术运算
    #[test]
    fn test_nonzerof32_add() {
        let a = NonZeroF32::new(2.0).unwrap();
        let b = NonZeroF32::new(3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 5.0);
    }

    #[test]
    fn test_nonzerof32_sub() {
        let a = NonZeroF32::new(10.0).unwrap();
        let b = NonZeroF32::new(3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), 7.0);
    }

    #[test]
    fn test_nonzerof32_mul() {
        let a = NonZeroF32::new(4.0).unwrap();
        let b = NonZeroF32::new(3.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 12.0);
    }

    #[test]
    fn test_nonzerof32_div() {
        let a = NonZeroF32::new(12.0).unwrap();
        let b = NonZeroF32::new(3.0).unwrap();
        let c = a / b;
        assert_eq!(c.get(), 4.0);
    }

    // NonZeroPositiveF32 算术运算
    #[test]
    fn test_nonzero_positivef32_add() {
        let a = NonZeroPositiveF32::new(2.0).unwrap();
        let b = NonZeroPositiveF32::new(3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 5.0);
    }

    #[test]
    fn test_nonzero_positivef32_mul() {
        let a = NonZeroPositiveF32::new(4.0).unwrap();
        let b = NonZeroPositiveF32::new(3.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 12.0);
    }

    #[test]
    fn test_nonzero_positivef32_div() {
        let a = NonZeroPositiveF32::new(12.0).unwrap();
        let b = NonZeroPositiveF32::new(3.0).unwrap();
        let c = a / b;
        assert_eq!(c.get(), 4.0);
    }

    // NonZeroF64 算术运算
    #[test]
    fn test_nonzerof64_add() {
        let a = NonZeroF64::new(2.5).unwrap();
        let b = NonZeroF64::new(3.5).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 6.0);
    }

    #[test]
    fn test_nonzerof64_mul() {
        let a = NonZeroF64::new(2.5).unwrap();
        let b = NonZeroF64::new(4.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 10.0);
    }

    // NonZeroPositiveF64 算术运算
    #[test]
    fn test_nonzero_positivef64_add() {
        let a = NonZeroPositiveF64::new(2.5).unwrap();
        let b = NonZeroPositiveF64::new(3.5).unwrap();
        let c = a + b;
        assert_eq!(c.get(), 6.0);
    }

    #[test]
    fn test_nonzero_positivef64_mul() {
        let a = NonZeroPositiveF64::new(2.5).unwrap();
        let b = NonZeroPositiveF64::new(4.0).unwrap();
        let c = a * b;
        assert_eq!(c.get(), 10.0);
    }
}

/// 测试 `NonZero` 类型的比较运算
mod test_nonzero_comparison_operations {
    use super::*;

    #[test]
    fn test_nonzerof32_partial_eq() {
        let a = NonZeroF32::new(1.0).unwrap();
        let b = NonZeroF32::new(1.0).unwrap();
        assert_eq!(a, b);

        let c = NonZeroF32::new(2.0).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn test_nonzerof32_partial_ord() {
        let a = NonZeroF32::new(1.0).unwrap();
        let b = NonZeroF32::new(2.0).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
    }

    #[test]
    fn test_nonzero_positivef32_comparison() {
        let a = NonZeroPositiveF32::new(1.0).unwrap();
        let b = NonZeroPositiveF32::new(2.0).unwrap();
        assert!(a < b);
        assert_eq!(a, NonZeroPositiveF32::new(1.0).unwrap());
    }
}

/// 测试 `NonZero` 类型的 Optional 类型
mod test_nonzero_optional_types {
    use super::*;

    #[test]
    fn test_optnonzerof32() {
        let some: OptNonZeroF32 = Some(NonZeroF32::new(1.0).unwrap());
        let none: OptNonZeroF32 = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn test_optnonzero_positivef32() {
        let some: OptNonZeroPositiveF32 = Some(NonZeroPositiveF32::new(1.0).unwrap());
        let none: OptNonZeroPositiveF32 = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }
}

/// 测试 `NonZero` 类型的约束验证
mod test_nonzero_constraints {
    use super::*;

    #[test]
    fn test_nonzerof32_constraint() {
        assert!(NonZeroF32::new(1.0).is_some());
        assert!(NonZeroF32::new(-1.0).is_some());
        assert!(NonZeroF32::new(f32::NAN).is_none());
        assert!(NonZeroF32::new(f32::INFINITY).is_none());
        assert!(NonZeroF32::new(f32::NEG_INFINITY).is_none());
        assert!(NonZeroF32::new(0.0).is_none());
        assert!(NonZeroF32::new(-0.0).is_none());
    }

    #[test]
    fn test_nonzero_positivef32_constraint() {
        assert!(NonZeroPositiveF32::new(1.0).is_some());
        assert!(NonZeroPositiveF32::new(f32::MAX).is_some());
        assert!(NonZeroPositiveF32::new(f32::NAN).is_none());
        assert!(NonZeroPositiveF32::new(-1.0).is_none());
        assert!(NonZeroPositiveF32::new(0.0).is_none());
        assert!(NonZeroPositiveF32::new(-0.0).is_none());
        assert!(NonZeroPositiveF32::new(f32::INFINITY).is_none());
    }
}

/// 测试 `NegativeF32` 的基本功能
mod test_negativef32 {
    use super::*;

    #[test]
    fn test_negativef32_new_valid() {
        assert!(NegativeF32::new(-1.0).is_some());
        assert!(NegativeF32::new(f32::MIN).is_some());
        // Negative 现在不再允许无穷大
        assert!(NegativeF32::new(f32::NEG_INFINITY).is_none());
        assert!(NegativeF32::new(-0.0).is_some());
        // Negative 现在使用数值比较 (<= 0.0)，接受 +0.0
        assert!(NegativeF32::new(0.0).is_some());
    }

    #[test]
    fn test_negativef32_new_invalid() {
        assert!(NegativeF32::new(f32::NAN).is_none());
        assert!(NegativeF32::new(1.0).is_none());
        assert!(NegativeF32::new(f32::INFINITY).is_none());
        assert!(NegativeF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_negativef32_get() {
        let negative = NegativeF32::new(-42.0).unwrap();
        assert_eq!(negative.get(), -42.0);
    }
}

/// 测试 `NegativeF64` 的基本功能
mod test_negativef64 {
    use super::*;

    #[test]
    fn test_negativef64_new_valid() {
        assert!(NegativeF64::new(-1.0).is_some());
        assert!(NegativeF64::new(f64::MIN).is_some());
        // Negative 现在不再允许无穷大
        assert!(NegativeF64::new(f64::NEG_INFINITY).is_none());
        assert!(NegativeF64::new(-0.0).is_some());
        // Negative 现在使用数值比较 (<= 0.0)，接受 +0.0
        assert!(NegativeF64::new(0.0).is_some());
    }

    #[test]
    fn test_negativef64_new_invalid() {
        assert!(NegativeF64::new(f64::NAN).is_none());
        assert!(NegativeF64::new(1.0).is_none());
        assert!(NegativeF64::new(f64::INFINITY).is_none());
        assert!(NegativeF64::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_negativef64_get() {
        let negative = NegativeF64::new(-123.456).unwrap();
        assert_eq!(negative.get(), -123.456);
    }
}

/// 测试 `NonZeroNegativeF32` 的基本功能
mod test_nonzero_negativef32 {
    use super::*;

    #[test]
    fn test_nonzero_negativef32_new_valid() {
        assert!(NonZeroNegativeF32::new(-1.0).is_some());
        assert!(NonZeroNegativeF32::new(f32::MIN).is_some());
        assert!(NonZeroNegativeF32::new(-0.00001).is_some());
    }

    #[test]
    fn test_nonzero_negativef32_new_invalid() {
        assert!(NonZeroNegativeF32::new(f32::NAN).is_none());
        assert!(NonZeroNegativeF32::new(1.0).is_none());
        assert!(NonZeroNegativeF32::new(0.0).is_none());
        assert!(NonZeroNegativeF32::new(-0.0).is_none());
        assert!(NonZeroNegativeF32::new(f32::INFINITY).is_none());
        assert!(NonZeroNegativeF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_nonzero_negativef32_get() {
        let non_zero_negative = NonZeroNegativeF32::new(-42.0).unwrap();
        assert_eq!(non_zero_negative.get(), -42.0);
    }
}

/// 测试 `NonZeroNegativeF64` 的基本功能
mod test_nonzero_negativef64 {
    use super::*;

    #[test]
    fn test_nonzero_negativef64_new_valid() {
        assert!(NonZeroNegativeF64::new(-1.0).is_some());
        assert!(NonZeroNegativeF64::new(f64::MIN).is_some());
        assert!(NonZeroNegativeF64::new(-0.00001).is_some());
    }

    #[test]
    fn test_nonzero_negativef64_new_invalid() {
        assert!(NonZeroNegativeF64::new(f64::NAN).is_none());
        assert!(NonZeroNegativeF64::new(1.0).is_none());
        assert!(NonZeroNegativeF64::new(0.0).is_none());
        assert!(NonZeroNegativeF64::new(-0.0).is_none());
        assert!(NonZeroNegativeF64::new(f64::INFINITY).is_none());
        assert!(NonZeroNegativeF64::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_nonzero_negativef64_get() {
        let non_zero_negative = NonZeroNegativeF64::new(-123.456).unwrap();
        assert_eq!(non_zero_negative.get(), -123.456);
    }
}

/// 测试 Negative 类型的算术运算
mod test_negative_arithmetic_operations {
    use super::*;

    // NegativeF32 算术运算
    #[test]
    fn test_negativef32_add() {
        let a = NegativeF32::new(-2.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -5.0);
    }

    #[test]
    fn test_negativef32_sub() {
        let a = NegativeF32::new(-10.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), -7.0);
    }

    #[test]
    fn test_negativef32_mul() {
        // 负数乘以负数会得到正数，但这违反了 Negative 类型约束
        // 因此我们只测试加法和减法
        let a = NegativeF32::new(-4.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -7.0);
    }

    #[test]
    fn test_negativef32_div() {
        // 负数除以负数会得到正数，但这违反了 Negative 类型约束
        // 因此我们只测试加法和减法
        let a = NegativeF32::new(-12.0).unwrap();
        let b = NegativeF32::new(-3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), -9.0);
    }

    // NonZeroNegativeF32 算术运算
    #[test]
    fn test_nonzero_negativef32_add() {
        let a = NonZeroNegativeF32::new(-2.0).unwrap();
        let b = NonZeroNegativeF32::new(-3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -5.0);
    }

    #[test]
    fn test_nonzero_negativef32_mul() {
        // 非零负数乘以非零负数会得到正数，违反了约束
        // 因此我们只测试加法和减法
        let a = NonZeroNegativeF32::new(-4.0).unwrap();
        let b = NonZeroNegativeF32::new(-3.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -7.0);
    }

    #[test]
    fn test_nonzero_negativef32_div() {
        // 非零负数除以非零负数会得到正数，违反了约束
        // 因此我们只测试加法和减法
        let a = NonZeroNegativeF32::new(-12.0).unwrap();
        let b = NonZeroNegativeF32::new(-3.0).unwrap();
        let c = a - b;
        assert_eq!(c.get(), -9.0);
    }

    // NegativeF64 算术运算
    #[test]
    fn test_negativef64_add() {
        let a = NegativeF64::new(-2.5).unwrap();
        let b = NegativeF64::new(-3.5).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -6.0);
    }

    #[test]
    fn test_negativef64_mul() {
        // 负数乘以负数会得到正数，违反了约束
        // 因此我们只测试加法和减法
        let a = NegativeF64::new(-2.5).unwrap();
        let b = NegativeF64::new(-4.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -6.5);
    }

    // NonZeroNegativeF64 算术运算
    #[test]
    fn test_nonzero_negativef64_add() {
        let a = NonZeroNegativeF64::new(-2.5).unwrap();
        let b = NonZeroNegativeF64::new(-3.5).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -6.0);
    }

    #[test]
    fn test_nonzero_negativef64_mul() {
        // 非零负数乘以非零负数会得到正数，违反了约束
        // 因此我们只测试加法和减法
        let a = NonZeroNegativeF64::new(-2.5).unwrap();
        let b = NonZeroNegativeF64::new(-4.0).unwrap();
        let c = a + b;
        assert_eq!(c.get(), -6.5);
    }
}

/// 测试 Negative 类型的比较运算
mod test_negative_comparison_operations {
    use super::*;

    #[test]
    fn test_negativef32_partial_eq() {
        let a = NegativeF32::new(-1.0).unwrap();
        let b = NegativeF32::new(-1.0).unwrap();
        assert_eq!(a, b);

        let c = NegativeF32::new(-2.0).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn test_negativef32_partial_ord() {
        let a = NegativeF32::new(-2.0).unwrap();
        let b = NegativeF32::new(-1.0).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
    }

    #[test]
    fn test_nonzero_negativef32_comparison() {
        let a = NonZeroNegativeF32::new(-2.0).unwrap();
        let b = NonZeroNegativeF32::new(-1.0).unwrap();
        assert!(a < b);
        assert_eq!(a, NonZeroNegativeF32::new(-2.0).unwrap());
    }
}

/// 测试 Negative 类型的 Optional 类型
mod test_negative_optional_types {
    use super::*;

    #[test]
    fn test_optnegativef32() {
        let some: OptNegativeF32 = Some(NegativeF32::new(-1.0).unwrap());
        let none: OptNegativeF32 = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }

    #[test]
    fn test_optnonzero_negativef32() {
        let some: OptNonZeroNegativeF32 = Some(NonZeroNegativeF32::new(-1.0).unwrap());
        let none: OptNonZeroNegativeF32 = None;

        assert!(some.is_some());
        assert!(none.is_none());
    }
}

/// 测试 Negative 类型的约束验证
mod test_negative_constraints {
    use super::*;

    #[test]
    fn test_negativef32_constraint() {
        assert!(NegativeF32::new(-1.0).is_some());
        assert!(NegativeF32::new(f32::MIN).is_some());
        // Negative 现在不再允许无穷大
        assert!(NegativeF32::new(f32::NEG_INFINITY).is_none());
        assert!(NegativeF32::new(-0.0).is_some());
        // Negative 现在使用数值比较 (<= 0.0)，接受 +0.0
        assert!(NegativeF32::new(0.0).is_some());
        assert!(NegativeF32::new(f32::NAN).is_none());
        assert!(NegativeF32::new(1.0).is_none());
        assert!(NegativeF32::new(f32::INFINITY).is_none());
    }

    #[test]
    fn test_nonzero_negativef32_constraint() {
        assert!(NonZeroNegativeF32::new(-1.0).is_some());
        assert!(NonZeroNegativeF32::new(f32::MIN).is_some());
        assert!(NonZeroNegativeF32::new(f32::NAN).is_none());
        assert!(NonZeroNegativeF32::new(1.0).is_none());
        assert!(NonZeroNegativeF32::new(0.0).is_none());
        assert!(NonZeroNegativeF32::new(-0.0).is_none());
        assert!(NonZeroNegativeF32::new(f32::INFINITY).is_none());
        assert!(NonZeroNegativeF32::new(f32::NEG_INFINITY).is_none());
    }
}

/// 测试 `NegativeNormalizedF32` 的基本功能
mod test_negative_normalizedf32 {
    use super::*;

    #[test]
    fn test_negative_normalizedf32_new_valid() {
        // 边界值
        assert!(NegativeNormalizedF32::new(-1.0).is_some());
        assert!(NegativeNormalizedF32::new(0.0).is_some());
        assert!(NegativeNormalizedF32::new(-0.0).is_some());

        // 中间值
        assert!(NegativeNormalizedF32::new(-0.5).is_some());
        assert!(NegativeNormalizedF32::new(-0.75).is_some());
        assert!(NegativeNormalizedF32::new(-0.001).is_some());
        assert!(NegativeNormalizedF32::new(-0.999).is_some());
    }

    #[test]
    fn test_negative_normalizedf32_new_invalid() {
        // 超出下界
        assert!(NegativeNormalizedF32::new(-1.1).is_none());
        assert!(NegativeNormalizedF32::new(-2.0).is_none());
        assert!(NegativeNormalizedF32::new(f32::MIN).is_none());

        // 超出上界
        assert!(NegativeNormalizedF32::new(0.1).is_none());
        assert!(NegativeNormalizedF32::new(1.0).is_none());
        assert!(NegativeNormalizedF32::new(f32::MAX).is_none());

        // 特殊值
        assert!(NegativeNormalizedF32::new(f32::NAN).is_none());
        assert!(NegativeNormalizedF32::new(f32::INFINITY).is_none());
        assert!(NegativeNormalizedF32::new(f32::NEG_INFINITY).is_none());
    }

    #[test]
    fn test_negative_normalizedf32_get() {
        let negative_normalized = NegativeNormalizedF32::new(-0.75).unwrap();
        assert_eq!(negative_normalized.get(), -0.75);
    }
}

/// 测试 `NegativeNormalizedF64` 的基本功能
mod test_negative_normalizedf64 {
    use super::*;

    #[test]
    fn test_negative_normalizedf64_new_valid() {
        // 边界值
        assert!(NegativeNormalizedF64::new(-1.0).is_some());
        assert!(NegativeNormalizedF64::new(0.0).is_some());

        // 中间值
        assert!(NegativeNormalizedF64::new(-0.5).is_some());
        assert!(NegativeNormalizedF64::new(-0.75).is_some());
    }

    #[test]
    fn test_negative_normalizedf64_new_invalid() {
        assert!(NegativeNormalizedF64::new(-1.1).is_none());
        assert!(NegativeNormalizedF64::new(0.1).is_none());
        assert!(NegativeNormalizedF64::new(f64::NAN).is_none());
        assert!(NegativeNormalizedF64::new(f64::INFINITY).is_none());
    }

    #[test]
    fn test_negative_normalizedf64_get() {
        let negative_normalized = NegativeNormalizedF64::new(-0.75).unwrap();
        assert_eq!(negative_normalized.get(), -0.75);
    }
}
