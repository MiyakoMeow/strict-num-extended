//! F32/F64 类型转换方法生成器模块

use proc_macro2::Ident;
use quote::{format_ident, quote};

use crate::config::TypeConfig;
use crate::generator::for_all_constraint_float_types;

/// Generates `try_into_f32` methods for all XXXF64 types
pub fn generate_try_into_f32_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // 只有 F64 类型才需要 try_into_f32
        if *float_type != "f64" {
            return quote! {};
        }

        // 生成对应的 F32 类型名称
        let f32_type_alias = make_type_alias(type_name, &format_ident!("f32"));

        // 为 F64 类型生成转换逻辑（直接使用 as 和 is_finite，避开 trait 限制）
        quote! {
            impl #type_alias {
                /// Attempts to convert to the corresponding F32 type
                ///
                /// # Errors
                ///
                /// Returns `Err(())` if:
                /// - The value is outside F32 range
                /// - Precision would be lost in the conversion
                /// - The converted value does not satisfy the target constraint
                #[must_use = "Return value may contain an error and should not be ignored"]
                pub const fn try_into_f32(self) -> Result<#f32_type_alias, ()> {
                    let value_f64 = self.value;
                    let value_f32 = value_f64 as f32;

                    // Check range: F32 is finite if within representable range
                    if !value_f32.is_finite() {
                        return Err(());
                    }

                    // Check precision: round-trip conversion should preserve value
                    if value_f32 as f64 != value_f64 {
                        return Err(());
                    }

                    // Use new_const to validate constraints - it is const and handles validation
                    Ok(#f32_type_alias::new_const(value_f32))
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}

/// Generates `as_f64` methods for all XXXF32 types
pub fn generate_as_f64_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // 只有 F32 类型才需要 as_f64
        if *float_type != "f32" {
            return quote! {};
        }

        // 生成对应的 F64 类型名称
        let f64_type_alias = make_type_alias(type_name, &format_ident!("f64"));

        // 为 F32 类型生成转换逻辑
        quote! {
            impl #type_alias {
                /// Converts to the corresponding F64 type
                ///
                /// Since F64 has a larger range than F32, this conversion
                /// is always safe in terms of range and precision.
                #[must_use]
                pub const fn as_f64(self) -> #f64_type_alias {
                    let value_f32 = self.value;
                    let value_f64 = value_f32 as f64;

                    // Use new_const to validate constraints
                    // Since F64 range is larger than F32, this should always succeed
                    #f64_type_alias::new_const(value_f64)
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}
