//! # From/TryFrom trait 实现生成模块
//!
//! 自动生成所有类型转换的标准库 trait 实现

use crate::config::TypeConfig;
use crate::generator::for_all_constraint_float_types;
use proc_macro2::Ident;
use quote::format_ident;
use quote::quote;

/// 生成所有 From/TryFrom 实现
pub fn generate_conversion_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let mut all_code = vec![];

    // 1. 约束类型 → 原语 (From)
    all_code.push(generate_constraint_to_primitive_from(config));

    // 2. 原语 → 约束类型 (TryFrom)
    all_code.push(generate_primitive_to_constraint_tryfrom(config));

    // 3. 约束类型 → 约束类型 (From/TryFrom)
    all_code.push(generate_constraint_to_constraint_traits(config));

    // 4. F32 → F64 (From)
    all_code.push(generate_f32_to_f64_from(config));

    // 5. F64 → F32 (TryFrom)
    all_code.push(generate_f64_to_f32_tryfrom(config));

    // 6. f32 → F64约束类型 (TryFrom)
    all_code.push(generate_f32_to_f64_constraint_tryfrom(config));

    // 7. f64 → F32约束类型 (TryFrom)
    all_code.push(generate_f64_to_f32_constraint_tryfrom(config));

    quote! { #(#all_code)* }
}

/// 判断源约束是否包含于目标约束（是否总是安全）
fn is_subset_constraint(
    src_lower: Option<f64>,
    src_upper: Option<f64>,
    src_excludes_zero: bool,
    dst_lower: Option<f64>,
    dst_upper: Option<f64>,
    dst_excludes_zero: bool,
) -> bool {
    // 1. 目标下界必须 <= 源下界
    let lower_contains = match (src_lower, dst_lower) {
        (Some(src), Some(dst)) => src >= dst,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => true,
    };

    // 2. 目标上界必须 >= 源上界
    let upper_contains = match (src_upper, dst_upper) {
        (Some(src), Some(dst)) => src <= dst,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => true,
    };

    // 3. 零排除要求兼容
    let zero_compatible = !dst_excludes_zero || src_excludes_zero;

    lower_contains && upper_contains && zero_compatible
}

/// 生成: 约束类型 → 原语 (From)
fn generate_constraint_to_primitive_from(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let alias = format_ident!("{}{}", type_name, float_type.to_string().to_uppercase());

        quote! {
            impl From<#alias> for #float_type {
                #[inline]
                fn from(value: #alias) -> Self {
                    value.get()
                }
            }
        }
    });

    quote! { #(#impls)* }
}

/// 生成: 原语 → 约束类型 (`TryFrom`)
fn generate_primitive_to_constraint_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let alias = format_ident!("{}{}", type_name, float_type.to_string().to_uppercase());

        quote! {
            impl TryFrom<#float_type> for #alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: #float_type) -> Result<Self, Self::Error> {
                    Self::new(value)
                }
            }
        }
    });

    quote! { #(#impls)* }
}

/// 生成: 约束类型 → 约束类型 (From/TryFrom)
fn generate_constraint_to_constraint_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let mut all_impls = vec![];

    // 为每种浮点类型生成转换
    for float_type in &["f32", "f64"] {
        let float_ident = Ident::new(float_type, proc_macro2::Span::call_site());

        // 收集所有该浮点类型的约束类型
        let types: Vec<_> = config
            .constraint_types
            .iter()
            .filter(|tt| tt.float_types.contains(&float_ident))
            .collect();

        // 为每对类型生成 From 或 TryFrom
        for src_type in &types {
            for dst_type in &types {
                if src_type.type_name.eq(&dst_type.type_name) {
                    continue; // 跳过相同类型
                }

                let src_alias = format_ident!(
                    "{}{}",
                    src_type.type_name,
                    float_type.to_string().to_uppercase()
                );
                let dst_alias = format_ident!(
                    "{}{}",
                    dst_type.type_name,
                    float_type.to_string().to_uppercase()
                );

                // 查找约束定义
                let src_constraint = config
                    .constraints
                    .iter()
                    .find(|c| c.name == src_type.constraint_name)
                    .unwrap();
                let dst_constraint = config
                    .constraints
                    .iter()
                    .find(|c| c.name == dst_type.constraint_name)
                    .unwrap();

                // 判断是否是子集关系
                let is_safe = is_subset_constraint(
                    src_constraint.bounds.lower,
                    src_constraint.bounds.upper,
                    src_constraint.excludes_zero,
                    dst_constraint.bounds.lower,
                    dst_constraint.bounds.upper,
                    dst_constraint.excludes_zero,
                );

                if is_safe {
                    // From 实现
                    all_impls.push(quote! {
                        impl From<#src_alias> for #dst_alias {
                            #[inline]
                            fn from(value: #src_alias) -> Self {
                                unsafe { Self::new_unchecked(value.get()) }
                            }
                        }
                    });
                } else {
                    // TryFrom 实现
                    all_impls.push(quote! {
                        impl TryFrom<#src_alias> for #dst_alias {
                            type Error = FloatError;

                            #[inline]
                            fn try_from(value: #src_alias) -> Result<Self, Self::Error> {
                                Self::new(value.get())
                            }
                        }
                    });
                }
            }
        }
    }

    quote! { #(#all_impls)* }
}

/// 生成: F32 → F64 (From)
fn generate_f32_to_f64_from(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if *float_type != "f32" {
            return quote! {};
        }

        let f32_alias = format_ident!("{}F32", type_name);
        let f64_alias = format_ident!("{}F64", type_name);

        quote! {
            impl From<#f32_alias> for #f64_alias {
                #[inline]
                fn from(value: #f32_alias) -> Self {
                    value.as_f64()
                }
            }
        }
    });

    quote! { #(#impls)* }
}

/// 生成: F64 → F32 (`TryFrom`)
fn generate_f64_to_f32_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if *float_type != "f64" {
            return quote! {};
        }

        let f32_alias = format_ident!("{}F32", type_name);
        let f64_alias = format_ident!("{}F64", type_name);

        quote! {
            impl TryFrom<#f64_alias> for #f32_alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: #f64_alias) -> Result<Self, Self::Error> {
                    value.try_into_f32()
                }
            }
        }
    });

    quote! { #(#impls)* }
}

/// 生成: f32 → F64约束类型 (TryFrom)
fn generate_f32_to_f64_constraint_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if float_type.to_string() != "f64" {
            return quote! {};
        }

        let f64_alias = format_ident!("{}F64", type_name);

        quote! {
            impl TryFrom<f32> for #f64_alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: f32) -> Result<Self, Self::Error> {
                    Self::new(value as f64)
                }
            }
        }
    });

    quote! { #(#impls)* }
}

/// 生成: f64 → F32约束类型 (TryFrom)
fn generate_f64_to_f32_constraint_tryfrom(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        if float_type.to_string() != "f32" {
            return quote! {};
        }

        let f32_alias = format_ident!("{}F32", type_name);

        quote! {
            impl TryFrom<f64> for #f32_alias {
                type Error = FloatError;

                #[inline]
                fn try_from(value: f64) -> Result<Self, Self::Error> {
                    Self::new(value as f32)
                }
            }
        }
    });

    quote! { #(#impls)* }
}
