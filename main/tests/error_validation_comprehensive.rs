//! 全面的错误验证测试
//!
//! 测试目标:
//! 1. 覆盖所有错误类型
//! 2. 测试边界情况和特殊值
//! 3. 验证错误消息准确性
//! 4. 确保编译时和运行时一致性

#![allow(clippy::unwrap_used)]

use strict_num_extended::*;

// ========== NaN 错误测试 ==========

mod test_nan_errors {
    use super::*;

    #[test]
    fn test_standard_nan() {
        assert!(matches!(FinF32::new(f32::NAN), Err(FloatError::NaN)));
        assert!(matches!(FinF64::new(f64::NAN), Err(FloatError::NaN)));
    }

    #[test]
    fn test_negative_nan() {
        assert!(matches!(FinF32::new(-f32::NAN), Err(FloatError::NaN)));
        assert!(matches!(FinF64::new(-f64::NAN), Err(FloatError::NaN)));
    }

    #[test]
    fn test_arithmetic_nan() {
        // ∞ - ∞ = NaN
        let nan_inf = f32::INFINITY - f32::INFINITY;
        assert!(matches!(FinF32::new(nan_inf), Err(FloatError::NaN)));
    }

    #[test]
    fn test_nan_in_all_types() {
        // PositiveF64 应该拒绝 NaN
        assert!(matches!(PositiveF64::new(f64::NAN), Err(FloatError::NaN)));

        // NegativeF64 应该拒绝 NaN
        assert!(matches!(NegativeF64::new(f64::NAN), Err(FloatError::NaN)));

        // NonZeroF64 应该拒绝 NaN
        assert!(matches!(NonZeroF64::new(f64::NAN), Err(FloatError::NaN)));

        // NormalizedF64 应该拒绝 NaN
        assert!(matches!(NormalizedF64::new(f64::NAN), Err(FloatError::NaN)));

        // SymmetricF64 应该拒绝 NaN
        assert!(matches!(SymmetricF64::new(f64::NAN), Err(FloatError::NaN)));
    }
}

// ========== 无穷大错误测试 ==========

mod test_infinity_errors {
    use super::*;

    #[test]
    fn test_positive_infinity() {
        assert!(matches!(
            FinF32::new(f32::INFINITY),
            Err(FloatError::PosInf)
        ));
        assert!(matches!(
            FinF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));
    }

    #[test]
    fn test_negative_infinity() {
        assert!(matches!(
            FinF32::new(f32::NEG_INFINITY),
            Err(FloatError::NegInf)
        ));
        assert!(matches!(
            FinF64::new(f64::NEG_INFINITY),
            Err(FloatError::NegInf)
        ));
    }

    #[test]
    fn test_arithmetic_infinity() {
        // 1.0 / 0.0 = +∞
        let inf = 1.0f32 / 0.0;
        assert!(matches!(FinF32::new(inf), Err(FloatError::PosInf)));

        // -1.0 / 0.0 = -∞
        let neg_inf = -1.0f32 / 0.0;
        assert!(matches!(FinF32::new(neg_inf), Err(FloatError::NegInf)));
    }

    #[test]
    fn test_overflow_infinity() {
        // f32::MAX * 2.0 溢出产生 +∞
        let overflow = f32::MAX * 2.0;
        assert!(matches!(FinF32::new(overflow), Err(FloatError::PosInf)));

        // f32::MIN * 2.0 溢出产生 -∞
        let underflow = f32::MIN * 2.0;
        assert!(matches!(FinF32::new(underflow), Err(FloatError::NegInf)));
    }

    #[test]
    fn test_infinity_in_all_types() {
        // PositiveF64 应该拒绝 +∞
        assert!(matches!(
            PositiveF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));

        // NegativeF64 应该拒绝 -∞
        assert!(matches!(
            NegativeF64::new(f64::NEG_INFINITY),
            Err(FloatError::NegInf)
        ));

        // NonZeroF64 应该拒绝 +∞
        assert!(matches!(
            NonZeroF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));

        // NormalizedF64 应该拒绝 +∞
        assert!(matches!(
            NormalizedF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));

        // SymmetricF64 应该拒绝 +∞
        assert!(matches!(
            SymmetricF64::new(f64::INFINITY),
            Err(FloatError::PosInf)
        ));
    }
}

// ========== 零值变体测试 ==========

mod test_zero_variants {
    use super::*;

    #[test]
    fn test_positive_zero() {
        // Positive 应该接受 +0.0
        assert!(PositiveF32::new(0.0).is_ok());
        assert!(PositiveF64::new(0.0).is_ok());

        // Negative 应该接受 +0.0
        assert!(NegativeF32::new(0.0).is_ok());
        assert!(NegativeF64::new(0.0).is_ok());

        // NonZero 应该拒绝 +0.0
        assert!(matches!(NonZeroF32::new(0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(NonZeroF64::new(0.0), Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_negative_zero() {
        // IEEE 754: -0.0 == 0.0
        assert_eq!(-0.0f32, 0.0f32);
        assert_eq!(-0.0f64, 0.0f64);

        // Positive 应该接受 -0.0（因为 -0.0 == 0.0）
        assert!(PositiveF32::new(-0.0).is_ok());
        assert!(PositiveF64::new(-0.0).is_ok());

        // Negative 应该接受 -0.0
        assert!(NegativeF32::new(-0.0).is_ok());
        assert!(NegativeF64::new(-0.0).is_ok());

        // NonZero 应该拒绝 -0.0（因为 -0.0 == 0.0，而 val != 0.0 为 false）
        assert!(matches!(NonZeroF32::new(-0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(NonZeroF64::new(-0.0), Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_zero_equality() {
        // 验证 +0.0 和 -0.0 在类型中是相等的
        let pos_zero = PositiveF32::new(0.0).unwrap();
        let neg_zero = PositiveF32::new(-0.0).unwrap();
        assert_eq!(pos_zero.get(), neg_zero.get());
    }

    #[test]
    fn test_nonzero_rejects_both_zeros() {
        // NonZeroPositive 应该拒绝 +0.0 和 -0.0
        assert!(matches!(
            NonZeroPositiveF32::new(0.0),
            Err(FloatError::OutOfRange)
        ));
        assert!(matches!(
            NonZeroPositiveF32::new(-0.0),
            Err(FloatError::OutOfRange)
        ));

        assert!(matches!(
            NonZeroNegativeF32::new(0.0),
            Err(FloatError::OutOfRange)
        ));
        assert!(matches!(
            NonZeroNegativeF32::new(-0.0),
            Err(FloatError::OutOfRange)
        ));
    }
}

// ========== 除零错误测试 ==========

mod test_division_by_zero_errors {
    use super::*;

    #[test]
    fn test_division_by_positive_zero() {
        let a = PositiveF64::new(10.0).unwrap();
        let zero = PositiveF64::new(0.0).unwrap();

        let result = a / zero;
        assert!(matches!(result, Err(FloatError::DivisionByZero)));
    }

    #[test]
    fn test_division_by_negative_zero() {
        let a = PositiveF64::new(10.0).unwrap();
        // 使用 unsafe 创建 -0.0
        let zero_neg = unsafe { PositiveF64::new_unchecked(-0.0) };

        let result = a / zero_neg;
        assert!(matches!(result, Err(FloatError::DivisionByZero)));
    }

    #[test]
    fn test_division_by_zero_all_types() {
        // FinF64
        let fin_a = FinF64::new(10.0).unwrap();
        let fin_zero = FinF64::new(0.0).unwrap();
        assert!(matches!(fin_a / fin_zero, Err(FloatError::DivisionByZero)));

        // NormalizedF64
        let norm_a = NormalizedF64::new(0.5).unwrap();
        let norm_zero = NormalizedF64::new(0.0).unwrap();
        assert!(matches!(
            norm_a / norm_zero,
            Err(FloatError::DivisionByZero)
        ));
    }

    #[test]
    fn test_nonzero_types_no_division_by_zero() {
        // NonZero 类型理论上不会除零，因为零值在创建时就被拒绝了
        let a = PositiveF64::new(10.0).unwrap();
        let b = NonZeroPositiveF64::new(2.0).unwrap();

        let result = a / b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 5.0);
    }
}

// ========== 精度损失测试 ==========

mod test_precision_loss {
    use super::*;

    #[test]
    fn test_exact_conversion() {
        // 整数和简单分数应该精确转换
        assert!(FinF64::new(3.0).unwrap().try_into_f32().is_ok());
        assert!(FinF64::new(0.5).unwrap().try_into_f32().is_ok());
        assert!(FinF64::new(-1.5).unwrap().try_into_f32().is_ok());
        assert!(FinF64::new(2.0).unwrap().try_into_f32().is_ok());
    }

    #[test]
    fn test_precision_loss_detection() {
        // 高精度小数会损失精度
        let precise = FinF64::new(1.234_567_890_123_456_7).unwrap();
        assert!(precise.try_into_f32().is_err());

        // 验证返回的错误类型
        let result = precise.try_into_f32();
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_range_overflow() {
        // 超出 f32 范围的大数
        let huge = FinF64::new(1e40).unwrap();
        assert!(huge.try_into_f32().is_err());

        // 验证返回的错误类型
        let result = huge.try_into_f32();
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_range_underflow() {
        // 超出 f32 范围的小数（负数大数）
        let tiny = FinF64::new(-1e40).unwrap();
        assert!(tiny.try_into_f32().is_err());

        // 验证返回的错误类型
        let result = tiny.try_into_f32();
        assert!(matches!(result, Err(FloatError::OutOfRange)));
    }

    #[test]
    fn test_roundtrip_conversion() {
        // 精确值应该能够往返转换
        let val_f64 = FinF64::new(2.5).unwrap();
        let val_f32 = val_f64.try_into_f32().unwrap();
        let back_f64: FinF64 = val_f32.into();
        assert_eq!(back_f64.get(), 2.5);
    }

    #[test]
    fn test_pi_conversion() {
        // f64::π 转换到 f32 会损失精度
        let pi_f64 = FinF64::new(std::f64::consts::PI).unwrap();
        assert!(pi_f64.try_into_f32().is_err());

        // f32::π 本身是有效的（不需要转换）
        let pi_f32 = FinF32::new(std::f32::consts::PI).unwrap();
        assert!(pi_f32.get().is_finite());
    }

    #[test]
    fn test_f32_boundary_conversion() {
        // f32::MAX 可以精确转换
        let max = FinF64::new(f32::MAX as f64).unwrap();
        assert!(max.try_into_f32().is_ok());

        // f32::MIN 可以精确转换
        let min = FinF64::new(f32::MIN as f64).unwrap();
        assert!(min.try_into_f32().is_ok());
    }
}

// ========== 溢出/下溢测试 ==========

mod test_overflow_underflow {
    use super::*;

    #[test]
    fn test_addition_overflow() {
        let a = PositiveF64::new(1e308).unwrap();
        let b = PositiveF64::new(1e308).unwrap();

        let result = a + b;
        // 应该返回 PosInf 错误
        assert!(matches!(result, Err(FloatError::PosInf)));
    }

    #[test]
    fn test_subtraction_underflow() {
        let a = NegativeF64::new(-1e308).unwrap();
        let b = PositiveF64::new(1e308).unwrap();

        // Negative - Positive 结果类型是 Fin（通过运算符重载推导）
        let result = a - b;
        // 应该返回 NegInf 错误
        assert!(matches!(result, Err(FloatError::NegInf)));
    }

    #[test]
    fn test_multiplication_overflow() {
        let a = PositiveF64::new(1e200).unwrap();
        let b = PositiveF64::new(1e200).unwrap();

        let result = a * b;
        // 应该返回 PosInf 错误
        assert!(matches!(result, Err(FloatError::PosInf)));
    }

    #[test]
    fn test_f32_overflow() {
        // f32 版本的溢出测试
        // f32::MAX 约 3.4e38，使用足够大的值导致溢出
        let a = PositiveF32::new(2e38).unwrap();
        let b = PositiveF32::new(2e38).unwrap();

        let result = a + b;
        // 2e38 + 2e38 = 4e38 > f32::MAX，应该溢出
        assert!(matches!(result, Err(FloatError::PosInf)));
    }
}

// ========== 边界值测试 ==========

mod test_boundary_values {
    use super::*;

    #[test]
    fn test_normalized_boundaries() {
        // Normalized: [0.0, 1.0]
        assert!(NormalizedF32::new(0.0).is_ok());
        assert!(NormalizedF32::new(1.0).is_ok());
        assert!(matches!(
            NormalizedF32::new(-0.001),
            Err(FloatError::OutOfRange)
        ));
        assert!(matches!(
            NormalizedF32::new(1.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_symmetric_boundaries() {
        // Symmetric: [-1.0, 1.0]
        assert!(SymmetricF32::new(-1.0).is_ok());
        assert!(SymmetricF32::new(1.0).is_ok());
        assert!(matches!(
            SymmetricF32::new(-1.001),
            Err(FloatError::OutOfRange)
        ));
        assert!(matches!(
            SymmetricF32::new(1.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_finite_boundaries() {
        // Fin 应该接受所有有限值
        assert!(FinF32::new(f32::MIN).is_ok());
        assert!(FinF32::new(f32::MAX).is_ok());
        assert!(FinF64::new(f64::MIN).is_ok());
        assert!(FinF64::new(f64::MAX).is_ok());
    }

    #[test]
    fn test_positive_boundaries() {
        // Positive: [0.0, +∞)
        assert!(PositiveF32::new(0.0).is_ok());
        assert!(PositiveF32::new(f32::MAX).is_ok());
        assert!(matches!(
            PositiveF32::new(-0.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_negative_boundaries() {
        // Negative: (-∞, 0.0]
        assert!(NegativeF32::new(0.0).is_ok());
        assert!(NegativeF32::new(f32::MIN).is_ok());
        assert!(matches!(
            NegativeF32::new(0.001),
            Err(FloatError::OutOfRange)
        ));
    }

    #[test]
    fn test_nonzero_boundaries() {
        // NonZero: (-∞, 0.0) ∪ (0.0, +∞)
        assert!(NonZeroF32::new(0.001).is_ok());
        assert!(NonZeroF32::new(-0.001).is_ok());
        assert!(matches!(NonZeroF32::new(0.0), Err(FloatError::OutOfRange)));
        assert!(matches!(NonZeroF32::new(-0.0), Err(FloatError::OutOfRange)));
    }
}

// ========== 错误消息测试 ==========

mod test_error_messages {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", FloatError::NaN),
            "value is NaN (Not a Number)"
        );
        assert_eq!(
            format!("{}", FloatError::PosInf),
            "value is positive infinity"
        );
        assert_eq!(
            format!("{}", FloatError::NegInf),
            "value is negative infinity"
        );
        assert_eq!(
            format!("{}", FloatError::OutOfRange),
            "value is outside the valid range for this type"
        );
        assert_eq!(
            format!("{}", FloatError::DivisionByZero),
            "division by zero"
        );
        assert_eq!(
            format!("{}", FloatError::NoneOperand),
            "right-hand side operand is None in Option arithmetic"
        );
    }

    #[test]
    fn test_error_debug() {
        assert!(format!("{:?}", FloatError::NaN).contains("NaN"));
        assert!(format!("{:?}", FloatError::PosInf).contains("PosInf"));
        assert!(format!("{:?}", FloatError::NegInf).contains("NegInf"));
        assert!(format!("{:?}", FloatError::OutOfRange).contains("OutOfRange"));
        assert!(format!("{:?}", FloatError::DivisionByZero).contains("DivisionByZero"));
        assert!(format!("{:?}", FloatError::NoneOperand).contains("NoneOperand"));
    }
}

// ========== NoneOperand 错误测试 ==========

mod test_none_operand_errors {
    use super::*;

    #[test]
    fn test_option_none_operand_all_operations() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let none: Option<NegativeF64> = None;

        // 加法（安全操作）返回 None
        let add_result: Option<FinF64> = A + none;
        assert!(add_result.is_none());

        // 乘法（危险操作）返回 Err
        let none_pos: Option<PositiveF64> = None;
        let mul_result: Result<PositiveF64, FloatError> = A * none_pos;
        assert!(matches!(mul_result, Err(FloatError::NoneOperand)));

        // 除法（危险操作）返回 Err
        let div_result: Result<PositiveF64, FloatError> = A / none_pos;
        assert!(matches!(div_result, Err(FloatError::NoneOperand)));
    }

    #[test]
    fn test_none_operand_error_message() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let none: Option<PositiveF64> = None;

        let result: Result<PositiveF64, FloatError> = A * none;
        assert!(matches!(result, Err(FloatError::NoneOperand)));

        if let Err(e) = result {
            assert_eq!(
                format!("{}", e),
                "right-hand side operand is None in Option arithmetic"
            );
        }
    }
}

// ========== TryFrom 一致性测试 ==========

mod test_tryfrom_consistency {
    use super::*;

    #[test]
    fn test_try_from_primitive_consistency() {
        // TryFrom<f32> 应该与 new() 返回相同的错误
        let result_new = PositiveF32::new(-1.0);
        let result_try: Result<PositiveF32, _> = PositiveF32::try_from(-1.0f32);

        match (result_new, result_try) {
            (Err(FloatError::OutOfRange), Err(FloatError::OutOfRange)) => {}
            _ => panic!("TryFrom 和 new() 应该返回相同的错误类型"),
        }
    }

    #[test]
    fn test_try_from_f64_to_f32_consistency() {
        // TryFrom<f32> for F32 类型应该调用 new() 并返回相同的错误
        // 使用 f32::MAX，这是一个有效值
        let result_new = PositiveF32::new(f32::MAX);
        let result_try: Result<PositiveF32, _> = PositiveF32::try_from(f32::MAX);

        // 两者都应该成功
        assert!(result_new.is_ok());
        assert!(result_try.is_ok());
    }
}
