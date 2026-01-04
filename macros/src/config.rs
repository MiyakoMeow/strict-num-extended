//! Configuration structure and parsing module
//!
//! Defines configuration structures for procedural macros and `TokenStream` parsing logic.

use proc_macro2::{Ident, Span};
use syn::{Expr, Lit, parse::Parse, parse::ParseStream};

// ============================================================================
// Configuration structure definitions
// ============================================================================

/// Main configuration structure.
pub struct TypeConfig {
    /// List of constraint definitions.
    pub constraints: Vec<ConstraintDef>,
    /// List of constraint type definitions.
    pub constraint_types: Vec<TypeDef>,
}

/// Single constraint definition.
pub struct ConstraintDef {
    /// Constraint name.
    pub name: Ident,
    /// Constraint documentation.
    pub doc: String,
    /// Validation expression.
    pub validate: String,
}

/// Type definition (single constraint).
pub enum TypeDef {
    /// Single constraint type.
    Single {
        /// Type name.
        type_name: Ident,
        /// List of floating-point types.
        float_types: Vec<Ident>,
        /// Constraint name.
        constraint_name: Ident,
    },
}

impl TypeDef {
    /// Get the type name
    pub const fn type_name(&self) -> &Ident {
        match self {
            TypeDef::Single { type_name, .. } => type_name,
        }
    }

    /// Get the list of floating-point types
    pub fn float_types(&self) -> &[Ident] {
        match self {
            TypeDef::Single { float_types, .. } => float_types,
        }
    }
}

// ============================================================================
// Parse trait implementations
// ============================================================================

impl Parse for TypeConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析方括号内容：[ ... ]
        let content;
        syn::bracketed!(content in input);

        let mut constraints = Vec::new();
        let mut constraint_types = Vec::new();

        // 解析每个类型定义
        while !content.is_empty() {
            // 解析 (TypeName, ["条件1", "条件2", ...])
            let paren_content;
            syn::parenthesized!(paren_content in content);

            let type_name: Ident = paren_content.parse()?;
            paren_content.parse::<syn::Token![,]>()?;

            let bracket_content;
            syn::bracketed!(bracket_content in &paren_content);

            // 解析条件列表
            let mut conditions = Vec::new();
            while !bracket_content.is_empty() {
                let expr: Expr = bracket_content.parse()?;
                let condition = match &expr {
                    Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }) => s.value(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            expr,
                            "Expected string literal for validation condition",
                        ));
                    }
                };
                conditions.push(condition);

                // 如果不是最后一个，解析逗号
                if !bracket_content.is_empty() {
                    bracket_content.parse::<syn::Token![,]>()?;
                }
            }

            // 自动添加 is_finite 检查，然后组合所有条件（AND 逻辑）
            let finite_check = "value.is_finite()";
            let mut all_conditions = vec![finite_check.to_string()];
            all_conditions.extend(conditions.clone());
            let validate_expr = all_conditions.join(" && ");

            // 生成约束定义
            let doc = generate_auto_doc(&type_name, &conditions);
            constraints.push(ConstraintDef {
                name: type_name.clone(),
                doc,
                validate: validate_expr.clone(),
            });

            // 生成类型定义（自动添加 f32 和 f64）
            let type_name_clone = type_name.clone();
            constraint_types.push(TypeDef::Single {
                type_name,
                float_types: vec![
                    Ident::new("f32", Span::call_site()),
                    Ident::new("f64", Span::call_site()),
                ],
                constraint_name: type_name_clone,
            });

            let _ = content.parse::<syn::Token![,]>();
        }

        Ok(TypeConfig {
            constraints,
            constraint_types,
        })
    }
}

// ============================================================================
// Simplified format helpers (新格式辅助函数)
// ============================================================================

/// 自动生成文档
fn generate_auto_doc(type_name: &Ident, conditions: &[String]) -> String {
    let name_str = type_name.to_string();

    let base_desc = match name_str.as_str() {
        "Fin" => "Finite floating-point value",
        "Positive" => "Positive floating-point value (> 0, finite)",
        "Negative" => "Negative floating-point value (< 0, finite)",
        "NonZero" => "Non-zero floating-point value",
        "Normalized" => "Normalized floating-point value (0.0 <= value <= 1.0)",
        "NegativeNormalized" => "Negative normalized floating-point value (-1.0 <= value <= 0.0)",
        "NonZeroPositive" => "Non-zero positive floating-point value (> 0, finite)",
        "NonZeroNegative" => "Non-zero negative floating-point value (< 0, finite)",
        _ => &format!("Constrained floating-point value: {}", name_str),
    };

    let conditions_str = conditions.join(" && ");
    format!("{}\n\nValidation: {}", base_desc, conditions_str)
}
