//! `FromStr` trait 实现测试
//!
//! 测试目标：
//! 1. 测试解析有效值的各种格式
//! 2. 测试科学计数法支持
//! 3. 测试错误处理和有意义的错误信息
//! 4. 验证原始字符串在错误中保留

#![allow(clippy::unwrap_used)]

use strict_num_extended::*;

mod test_basic_parsing {
    use super::*;

    #[test]
    fn test_parse_simple_decimal() {
        let value: FinF32 = "3.14".parse().unwrap();
        assert_eq!(value.get(), 3.14);
    }

    #[test]
    fn test_parse_integer() {
        let value: FinF64 = "42".parse().unwrap();
        assert_eq!(value.get(), 42.0);
    }

    #[test]
    fn test_parse_negative() {
        let value: FinF32 = "-3.14".parse().unwrap();
        assert_eq!(value.get(), -3.14);
    }

    #[test]
    fn test_parse_zero() {
        let value: FinF64 = "0.0".parse().unwrap();
        assert_eq!(value.get(), 0.0);
    }
}

mod test_scientific_notation {
    use super::*;

    #[test]
    fn test_parse_scientific_lowercase() {
        let value: FinF32 = "1.5e2".parse().unwrap();
        assert_eq!(value.get(), 150.0);
    }

    #[test]
    fn test_parse_scientific_uppercase() {
        let value: FinF64 = "2.5E-3".parse().unwrap();
        assert_eq!(value.get(), 0.0025);
    }

    #[test]
    fn test_parse_scientific_integer() {
        let value: FinF32 = "1e10".parse().unwrap();
        assert_eq!(value.get(), 10_000_000_000.0);
    }

    #[test]
    fn test_parse_scientific_negative() {
        let value: FinF64 = "-1.5e2".parse().unwrap();
        assert_eq!(value.get(), -150.0);
    }

    #[test]
    fn test_parse_scientific_small_positive() {
        let value: FinF32 = "1.5e-10".parse().unwrap();
        assert_eq!(value.get(), 1.5e-10);
    }
}

mod test_constraint_validation {
    use super::*;

    #[test]
    fn test_positive_rejects_negative() {
        let result: Result<PositiveF32, _> = "-1.0".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_normalized_rejects_out_of_range() {
        let result: Result<NormalizedF64, _> = "1.5".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_normalized_rejects_negative() {
        let result: Result<NormalizedF32, _> = "-0.5".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_nonzero_rejects_zero() {
        let result: Result<NonZeroF32, _> = "0.0".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_nonzero_positive_rejects_negative() {
        let result: Result<NonZeroPositiveF64, _> = "-1.0".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::OutOfRange))
        ));
    }

    #[test]
    fn test_symmetric_rejects_out_of_range() {
        let result: Result<SymmetricF32, _> = "1.5".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::OutOfRange))
        ));
    }
}

mod test_special_values {
    use super::*;

    #[test]
    fn test_parse_nan_rejected() {
        let result: Result<FinF32, _> = "NaN".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::NaN))
        ));
    }

    #[test]
    fn test_parse_infinity_rejected() {
        let result: Result<FinF64, _> = "inf".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::PosInf))
        ));
    }

    #[test]
    fn test_parse_negative_infinity_rejected() {
        let result: Result<FinF32, _> = "-inf".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::NegInf))
        ));
    }

    #[test]
    fn test_parse_infinity_capitalized() {
        let result: Result<FinF64, _> = "Inf".parse();
        assert!(matches!(
            result,
            Err(FloatParseError::ValidationFailed(FloatError::PosInf))
        ));
    }
}

mod test_error_messages {
    use super::*;

    #[test]
    fn test_invalid_float_error_message() {
        let result: Result<FinF32, _> = "not_a_number".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("not_a_number"));
        assert!(msg.contains("failed to parse"));
    }

    #[test]
    fn test_invalid_float_error_message_empty() {
        let result: Result<FinF32, _> = "".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("failed to parse"));
    }

    #[test]
    fn test_out_of_range_error_message() {
        let result: Result<PositiveF32, _> = "-1.0".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("outside the valid range"));
    }

    #[test]
    fn test_nan_error_message() {
        let result: Result<FinF32, _> = "NaN".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("NaN"));
        assert!(msg.contains("Not a Number"));
    }

    #[test]
    fn test_infinity_error_message() {
        let result: Result<FinF64, _> = "inf".parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("positive infinity"));
    }
}

mod test_error_preserves_input {
    use super::*;

    #[test]
    fn test_invalid_float_contains_original_string() {
        let result: Result<FinF32, _> = "abc123".parse();
        if let Err(FloatParseError::InvalidFloat { input }) = result {
            assert_eq!(input, "abc123");
        } else {
            panic!("Expected InvalidFloat error with input");
        }
    }

    #[test]
    fn test_invalid_float_preserves_special_chars() {
        let result: Result<FinF32, _> = "$%^&*".parse();
        if let Err(FloatParseError::InvalidFloat { input }) = result {
            assert_eq!(input, "$%^&*");
        } else {
            panic!("Expected InvalidFloat error with special characters");
        }
    }

    #[test]
    fn test_validation_failed_wraps_float_error() {
        let result: Result<PositiveF32, _> = "-1.0".parse();
        if let Err(FloatParseError::ValidationFailed(FloatError::OutOfRange)) = result {
            // 正确包装了 OutOfRange 错误
        } else {
            panic!("Expected ValidationFailed(OutOfRange) error");
        }
    }

    #[test]
    fn test_nan_validation_failed() {
        let result: Result<FinF32, _> = "NaN".parse();
        if let Err(FloatParseError::ValidationFailed(FloatError::NaN)) = result {
            // 正确包装了 NaN 错误
        } else {
            panic!("Expected ValidationFailed(NaN) error");
        }
    }

    #[test]
    fn test_infinity_validation_failed() {
        let result: Result<FinF64, _> = "inf".parse();
        if let Err(FloatParseError::ValidationFailed(FloatError::PosInf)) = result {
            // 正确包装了 PosInf 错误
        } else {
            panic!("Expected ValidationFailed(PosInf) error");
        }
    }
}

mod test_all_types {
    use super::*;

    #[test]
    fn test_parse_all_f32_types() {
        assert!("3.14".parse::<FinF32>().is_ok());
        assert!("3.14".parse::<PositiveF32>().is_ok());
        assert!("-3.14".parse::<NegativeF32>().is_ok());
        assert!("3.14".parse::<NonZeroF32>().is_ok());
        assert!("0.5".parse::<NormalizedF32>().is_ok());
        assert!("0.5".parse::<SymmetricF32>().is_ok());
    }

    #[test]
    fn test_parse_all_f64_types() {
        assert!("3.14".parse::<FinF64>().is_ok());
        assert!("3.14".parse::<PositiveF64>().is_ok());
        assert!("-3.14".parse::<NegativeF64>().is_ok());
        assert!("3.14".parse::<NonZeroF64>().is_ok());
        assert!("0.5".parse::<NormalizedF64>().is_ok());
        assert!("0.5".parse::<SymmetricF64>().is_ok());
    }

    #[test]
    fn test_parse_nonzero_positive_types() {
        assert!("1.5".parse::<NonZeroPositiveF32>().is_ok());
        assert!("1.5".parse::<NonZeroPositiveF64>().is_ok());
    }

    #[test]
    fn test_parse_nonzero_negative_types() {
        assert!("-1.5".parse::<NonZeroNegativeF32>().is_ok());
        assert!("-1.5".parse::<NonZeroNegativeF64>().is_ok());
    }

    #[test]
    fn test_parse_negative_normalized_types() {
        assert!("-0.5".parse::<NegativeNormalizedF32>().is_ok());
        assert!("-0.5".parse::<NegativeNormalizedF64>().is_ok());
    }
}

mod test_edge_cases {
    use super::*;

    #[test]
    fn test_parse_very_small_number() {
        let value: FinF32 = "1e-38".parse().unwrap();
        assert!(value.get() > 0.0);
        assert!(value.get() < 1e-37);
    }

    #[test]
    fn test_parse_very_large_number() {
        let value: FinF64 = "1e308".parse().unwrap();
        assert!(value.get() > 1e307);
    }

    #[test]
    fn test_parse_zero_variations() {
        assert!("0".parse::<FinF32>().is_ok());
        assert!("0.0".parse::<FinF32>().is_ok());
        assert!("-0".parse::<FinF32>().is_ok());
        assert!("-0.0".parse::<FinF32>().is_ok());
        assert!("0e0".parse::<FinF32>().is_ok());
    }

    #[test]
    fn test_parse_whitespace_rejected() {
        // 标准库的 from_str 会拒绝前后有空格的字符串
        let result: Result<FinF32, _> = " 3.14".parse();
        assert!(matches!(result, Err(FloatParseError::InvalidFloat { .. })));

        let result: Result<FinF32, _> = "3.14 ".parse();
        assert!(matches!(result, Err(FloatParseError::InvalidFloat { .. })));
    }
}
