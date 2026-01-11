//! Constants generation module
//!
//! Generates mathematical constants for all constraint types.

use proc_macro2::{Ident, Span};
use quote::quote;

use crate::config::{ConstraintDef, TypeConfig};
use crate::generator::for_all_constraint_float_types;

/// 常量定义
struct ConstantDef {
    /// 常量名称（如 "ZERO", "PI"）
    name: &'static str,
    /// 文档注释
    doc: &'static str,
    /// f32 值表达式（Option 为 None 表示使用字面量）
    f32_expr: Option<&'static str>,
    /// f64 值表达式（Option 为 None 表示使用字面量）
    f64_expr: Option<&'static str>,
    /// 字面量值（用于边界检查）
    literal_value: f64,
}

/// 所有常量定义
const ALL_CONSTANTS: &[ConstantDef] = &[
    // ========== 基础常量 ==========
    ConstantDef {
        name: "ZERO",
        doc: "Zero (0.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 0.0,
    },
    ConstantDef {
        name: "ONE",
        doc: "One (1.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 1.0,
    },
    ConstantDef {
        name: "NEG_ONE",
        doc: "Negative one (-1.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -1.0,
    },
    ConstantDef {
        name: "TWO",
        doc: "Two (2.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 2.0,
    },
    ConstantDef {
        name: "NEG_TWO",
        doc: "Negative two (-2.0)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -2.0,
    },
    ConstantDef {
        name: "HALF",
        doc: "Half (0.5)",
        f32_expr: None,
        f64_expr: None,
        literal_value: 0.5,
    },
    ConstantDef {
        name: "NEG_HALF",
        doc: "Negative half (-0.5)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -0.5,
    },
    // ========== 数学常数 ==========
    ConstantDef {
        name: "PI",
        doc: "Pi (π)",
        f32_expr: Some("core::f32::consts::PI"),
        f64_expr: Some("core::f64::consts::PI"),
        literal_value: core::f64::consts::PI,
    },
    ConstantDef {
        name: "NEG_PI",
        doc: "Negative pi (-π)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -core::f64::consts::PI,
    },
    ConstantDef {
        name: "E",
        doc: "Euler's number (e)",
        f32_expr: Some("core::f32::consts::E"),
        f64_expr: Some("core::f64::consts::E"),
        literal_value: core::f64::consts::E,
    },
    ConstantDef {
        name: "NEG_E",
        doc: "Negative Euler's number (-e)",
        f32_expr: None,
        f64_expr: None,
        literal_value: -core::f64::consts::E,
    },
    // ========== π 分数常量 ==========
    ConstantDef {
        name: "FRAC_1_PI",
        doc: "1/π",
        f32_expr: Some("core::f32::consts::FRAC_1_PI"),
        f64_expr: Some("core::f64::consts::FRAC_1_PI"),
        literal_value: core::f64::consts::FRAC_1_PI,
    },
    ConstantDef {
        name: "FRAC_2_PI",
        doc: "2/π",
        f32_expr: Some("core::f32::consts::FRAC_2_PI"),
        f64_expr: Some("core::f64::consts::FRAC_2_PI"),
        literal_value: core::f64::consts::FRAC_2_PI,
    },
    ConstantDef {
        name: "FRAC_PI_2",
        doc: "π/2",
        f32_expr: Some("core::f32::consts::FRAC_PI_2"),
        f64_expr: Some("core::f64::consts::FRAC_PI_2"),
        literal_value: core::f64::consts::FRAC_PI_2,
    },
    ConstantDef {
        name: "FRAC_PI_3",
        doc: "π/3",
        f32_expr: Some("core::f32::consts::FRAC_PI_3"),
        f64_expr: Some("core::f64::consts::FRAC_PI_3"),
        literal_value: core::f64::consts::FRAC_PI_3,
    },
    ConstantDef {
        name: "FRAC_PI_4",
        doc: "π/4",
        f32_expr: Some("core::f32::consts::FRAC_PI_4"),
        f64_expr: Some("core::f64::consts::FRAC_PI_4"),
        literal_value: core::f64::consts::FRAC_PI_4,
    },
    ConstantDef {
        name: "FRAC_PI_6",
        doc: "π/6",
        f32_expr: Some("core::f32::consts::FRAC_PI_6"),
        f64_expr: Some("core::f64::consts::FRAC_PI_6"),
        literal_value: core::f64::consts::FRAC_PI_6,
    },
    ConstantDef {
        name: "FRAC_PI_8",
        doc: "π/8",
        f32_expr: Some("core::f32::consts::FRAC_PI_8"),
        f64_expr: Some("core::f64::consts::FRAC_PI_8"),
        literal_value: core::f64::consts::FRAC_PI_8,
    },
];

/// 检查常量值是否符合类型约束
fn constant_matches_constraint(value: f64, constraint_def: &ConstraintDef) -> bool {
    // 1. 检查是否有限（所有常量都是有限的，这个检查总是通过）
    if !value.is_finite() {
        return false;
    }

    // 2. 检查边界
    if let Some(lower) = constraint_def.bounds.lower {
        if value < lower {
            return false;
        }
    }
    if let Some(upper) = constraint_def.bounds.upper {
        if value > upper {
            return false;
        }
    }

    // 3. 检查零排除
    if constraint_def.excludes_zero && value == 0.0 {
        return false;
    }

    true
}

/// 生成所有类型的常量
pub fn generate_constants(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, constraint_def| {
        let struct_name = crate::generator::make_type_alias(type_name, float_type);

        // 过滤出适用于该类型的常量
        let applicable_constants: Vec<_> = ALL_CONSTANTS
            .iter()
            .filter(|c| constant_matches_constraint(c.literal_value, constraint_def))
            .collect();

        if applicable_constants.is_empty() {
            return quote! {};
        }

        // 为每个常量生成代码
        let constant_defs = applicable_constants.iter().map(|const_def| {
            let name = Ident::new(const_def.name, Span::call_site());
            let doc = const_def.doc;

            // 获取值表达式
            let value_expr = if *float_type == "f32" {
                const_def.f32_expr.map_or_else(
                    || {
                        let v = const_def.literal_value as f32;
                        quote! { #v }
                    },
                    |expr| {
                        // 解析表达式路径
                        expr.parse().expect("Invalid f32 expression")
                    },
                )
            } else {
                const_def.f64_expr.map_or_else(
                    || {
                        let v = const_def.literal_value;
                        quote! { #v }
                    },
                    |expr| {
                        // 解析表达式路径
                        expr.parse().expect("Invalid f64 expression")
                    },
                )
            };

            quote! {
                #[doc = #doc]
                #[must_use]
                pub const #name: Self = unsafe { Self::new_unchecked(#value_expr) };
            }
        });

        quote! {
            impl #struct_name {
                #(#constant_defs)*
            }
        }
    });

    quote! {
        #(#impls)*
    }
}
