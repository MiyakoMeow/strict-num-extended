//! # `FromStr` trait implementation generation module
//!
//! Automatically generates `FromStr` implementations for all constraint types

use crate::config::TypeConfig;
use crate::generator::for_all_constraint_float_types;
use quote::quote;

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
