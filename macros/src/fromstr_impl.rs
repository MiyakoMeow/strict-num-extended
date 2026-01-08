//! # `FromStr` trait implementation generation module
//!
//! Automatically generates `FromStr` implementations for all constraint types

use crate::config::TypeConfig;
use crate::generator::for_all_constraint_float_types;
use quote::quote;

/// Generate `FloatParseError` type and its trait implementations
pub fn generate_parse_error_type() -> proc_macro2::TokenStream {
    quote! {
        /// 字符串解析错误
        ///
        /// 包含两种可能的错误：
        /// 1. 字符串无法解析为浮点数
        /// 2. 解析成功但验证失败（包装 FloatError）
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum FloatParseError {
            /// 字符串无法解析为有效的浮点数
            InvalidFloat {
                /// 原始输入字符串
                input: String,
            },
            /// 解析成功但值验证失败
            ValidationFailed(FloatError),
        }

        impl core::fmt::Display for FloatParseError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    FloatParseError::InvalidFloat { input } => {
                        write!(f, "failed to parse '{}' as a floating-point number", input)
                    }
                    FloatParseError::ValidationFailed(e) => write!(f, "{}", e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for FloatParseError {}
    }
}

/// Generate all `FromStr` implementations
pub fn generate_fromstr_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = crate::generator::make_type_alias(type_name, float_type);

        quote! {
            impl core::str::FromStr for #struct_name {
                type Err = FloatParseError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    // 1. 使用标准库解析（自动支持科学计数法）
                    let value = #float_type::from_str(s).map_err(|_| FloatParseError::InvalidFloat {
                        input: s.to_string(),
                    })?;

                    // 2. 验证约束（复用现有 new() 逻辑）
                    Self::new(value).map_err(FloatParseError::ValidationFailed)
                }
            }
        }
    });

    quote! { #(#impls)* }
}
