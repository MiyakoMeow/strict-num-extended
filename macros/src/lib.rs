//! # Proc Macro 实现
//!
//! 为 strict-num-extended 提供完整的过程宏代码生成

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Expr, parse::Parse, parse::ParseStream, parse_macro_input, spanned::Spanned};

// ============================================================================
// 配置结构体定义
// ============================================================================

/// 主配置结构
struct TypeConfig {
    constraints: Vec<ConstraintDef>,
    constraint_types: Vec<TypeDef>,
    features: FeaturesConfig,
}

/// 单个约束定义
struct ConstraintDef {
    name: Ident,
    doc: String,
    validate: String,
}

/// 类型定义（单约束或组合约束）
enum TypeDef {
    Single {
        type_name: Ident,
        float_types: Vec<Ident>,
        constraint_name: Ident,
    },
    Combined {
        type_name: Ident,
        float_types: Vec<Ident>,
        constraints: Vec<Ident>,
    },
}

/// 特性配置
struct FeaturesConfig {
    impl_traits: Vec<Ident>,
    generate_option_types: bool,
    generate_new_const: bool,
}

// ============================================================================
// Parse trait 实现
// ============================================================================

impl Parse for TypeConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 解析大括号包围的内容
        let content;
        syn::braced!(content in input);

        let mut constraints = Vec::new();
        let mut constraint_types = Vec::new();
        let mut impl_traits = Vec::new();
        let mut generate_option_types = true;
        let mut generate_new_const = true;

        // 解析各个字段
        while !content.is_empty() {
            let ident: Ident = content.parse()?;

            match ident.to_string().as_str() {
                "constraints" => {
                    content.parse::<syn::Token![:]>()?;
                    constraints = parse_constraints(&content)?;
                }
                "constraint_types" => {
                    content.parse::<syn::Token![:]>()?;
                    constraint_types = parse_constraint_types(&content)?;
                }
                "features" => {
                    content.parse::<syn::Token![:]>()?;
                    let feature_content;
                    syn::braced!(feature_content in content);

                    while !feature_content.is_empty() {
                        let feature_ident: Ident = feature_content.parse()?;
                        feature_content.parse::<syn::Token![:]>()?;

                        match feature_ident.to_string().as_str() {
                            "impl_traits" => {
                                impl_traits = parse_ident_list(&feature_content)?;
                            }
                            "generate_option_types" => {
                                let value: Expr = feature_content.parse()?;
                                generate_option_types = parse_bool_expr(&value)?;
                            }
                            "generate_new_const" => {
                                let value: Expr = feature_content.parse()?;
                                generate_new_const = parse_bool_expr(&value)?;
                            }
                            _ => {
                                return Err(syn::Error::new(
                                    feature_ident.span(),
                                    format!("Unknown feature: {}", feature_ident),
                                ));
                            }
                        }

                        let _ = feature_content.parse::<syn::Token![,]>();
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown field: {}", ident),
                    ));
                }
            }

            let _ = content.parse::<syn::Token![,]>();
        }

        Ok(TypeConfig {
            constraints,
            constraint_types,
            features: FeaturesConfig {
                impl_traits,
                generate_option_types,
                generate_new_const,
            },
        })
    }
}

/// 解析约束定义列表
fn parse_constraints(input: ParseStream) -> syn::Result<Vec<ConstraintDef>> {
    let mut constraints = Vec::new();
    let content;
    syn::bracketed!(content in input);

    while !content.is_empty() {
        let name: Ident = content.parse()?;
        let field_content;
        syn::braced!(field_content in content);

        let mut doc = String::new();
        let mut validate = None;

        while !field_content.is_empty() {
            let ident: Ident = field_content.parse()?;
            field_content.parse::<syn::Token![:]>()?;

            match ident.to_string().as_str() {
                "doc" => {
                    if let Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = field_content.parse()?
                    {
                        doc = s.value();
                    }
                }
                "validate" => {
                    if let Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = field_content.parse()?
                    {
                        validate = Some(s.value());
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown field in constraint: {}", ident),
                    ));
                }
            }

            let _ = field_content.parse::<syn::Token![,]>();
        }

        let validate = validate
            .ok_or_else(|| syn::Error::new(name.span(), "Missing validate field in constraint"))?;

        constraints.push(ConstraintDef {
            name,
            doc,
            validate,
        });

        let _ = content.parse::<syn::Token![,]>();
    }

    Ok(constraints)
}

/// 解析约束类型定义（统一使用方括号）
fn parse_constraint_types(input: ParseStream) -> syn::Result<Vec<TypeDef>> {
    let mut types = Vec::new();
    let content;
    syn::bracketed!(content in input);

    while !content.is_empty() {
        // 解析 (Name, [f32, f64], [Constraint1, Constraint2, ...])
        let paren_content;
        syn::parenthesized!(paren_content in content);

        let type_name: Ident = paren_content.parse()?;
        paren_content.parse::<syn::Token![,]>()?;

        let float_types = parse_ident_list(&paren_content)?;
        paren_content.parse::<syn::Token![,]>()?;

        // 始终解析为列表（单约束也是列表）
        let constraints = parse_ident_list(&paren_content)?;

        // 根据约束数量决定是 Single 还是 Combined
        if constraints.len() == 1 {
            // 单约束类型
            let constraint_name = &constraints[0];
            types.push(TypeDef::Single {
                type_name,
                float_types,
                constraint_name: constraint_name.clone(),
            });
        } else {
            // 组合约束类型
            types.push(TypeDef::Combined {
                type_name,
                float_types,
                constraints,
            });
        }

        let _ = content.parse::<syn::Token![,]>();
    }

    Ok(types)
}

/// 解析标识符列表
fn parse_ident_list(input: ParseStream) -> syn::Result<Vec<Ident>> {
    let mut idents = Vec::new();
    let content;
    syn::bracketed!(content in input);

    while !content.is_empty() {
        let ident: Ident = content.parse()?;
        idents.push(ident);
        let _ = content.parse::<syn::Token![,]>();
    }

    Ok(idents)
}

/// 从表达式解析布尔值
fn parse_bool_expr(expr: &Expr) -> syn::Result<bool> {
    match expr {
        Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Bool(lit_bool),
            ..
        }) => Ok(lit_bool.value),
        _ => Err(syn::Error::new(expr.span(), "Expected boolean literal")),
    }
}

// ============================================================================
// 主宏：generate_constrained_types
// ============================================================================

/// 主宏：根据配置生成所有代码
#[proc_macro]
pub fn generate_constrained_types(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TypeConfig);

    // 收集所有需要生成的代码
    let mut all_code = Vec::new();

    // 1. 生成 Constraint trait
    all_code.push(generate_constraint_trait());

    // 2. 生成约束标记类型
    all_code.push(generate_constraint_markers(&config));

    // 3. 生成 Constrained 结构体
    all_code.push(generate_constrained_struct());

    // 4. 生成比较和格式化 trait
    all_code.push(generate_comparison_traits());

    // 5. 生成算术运算
    if config
        .features
        .impl_traits
        .iter()
        .any(|t| matches!(t.to_string().as_str(), "Add" | "Sub" | "Mul" | "Div"))
    {
        all_code.push(generate_arithmetic_impls(&config));
    }

    // 6. 生成类型别名
    all_code.push(generate_type_aliases(&config));

    // 7. 生成 new_const 方法
    if config.features.generate_new_const {
        all_code.push(generate_new_const_methods(&config));
    }

    // 组合所有代码
    let expanded = quote! {
        #(#all_code)*
    };

    TokenStream::from(expanded)
}

// ============================================================================
// 辅助宏实现
// ============================================================================

/// 生成 Constraint trait
fn generate_constraint_trait() -> TokenStream2 {
    quote! {
        /// 约束类型标记 trait
        pub trait Constraint {
            /// 基础类型（f32 或 f64）
            type Base;

            /// 验证值是否满足约束
            ///
            /// 返回 `true` 表示值满足约束条件，`false` 表示不满足。
            fn validate(value: Self::Base) -> bool;
        }
    }
}

/// 生成约束标记类型
fn generate_constraint_markers(config: &TypeConfig) -> TokenStream2 {
    let mut markers = Vec::new();
    let mut impls = Vec::new();

    for constraint in &config.constraints {
        let name = &constraint.name;
        let doc = &constraint.doc;

        // 将字符串转换为表达式
        let validate: Expr = syn::parse_str(&constraint.validate)
            .unwrap_or_else(|_| panic!("Invalid validate expression: {}", constraint.validate));

        // 生成标记类型
        markers.push(quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy)]
            pub struct #name<F = ()> {
                _marker: std::marker::PhantomData<F>,
            }
        });

        // 生成 f32 的实现
        impls.push(quote! {
            impl Constraint for #name<f32> {
                type Base = f32;

                fn validate(value: Self::Base) -> bool {
                    #validate
                }
            }
        });

        // 生成 f64 的实现
        impls.push(quote! {
            impl Constraint for #name<f64> {
                type Base = f64;

                fn validate(value: Self::Base) -> bool {
                    #validate
                }
            }
        });
    }

    // 生成元组组合约束实现
    let tuple_impls = generate_tuple_constraints();

    quote! {
        // 约束标记类型
        #(#markers)*

        // Constraint trait 实现
        #(#impls)*

        // 元组组合约束
        #tuple_impls
    }
}

/// 生成元组组合约束
fn generate_tuple_constraints() -> TokenStream2 {
    quote! {
        /// 单元素元组 (C1,)
        impl<T, C1> Constraint for (C1,)
        where
            T: Copy,
            C1: Constraint<Base = T>,
        {
            type Base = T;

            fn validate(value: Self::Base) -> bool {
                C1::validate(value)
            }
        }

        /// 双元素元组 (C1, C2)
        impl<T, C1, C2> Constraint for (C1, C2)
        where
            T: Copy,
            C1: Constraint<Base = T>,
            C2: Constraint<Base = T>,
        {
            type Base = T;

            fn validate(value: Self::Base) -> bool {
                C1::validate(value) && C2::validate(value)
            }
        }

        /// 三元素元组 (C1, C2, C3)
        impl<T, C1, C2, C3> Constraint for (C1, C2, C3)
        where
            T: Copy,
            C1: Constraint<Base = T>,
            C2: Constraint<Base = T>,
            C3: Constraint<Base = T>,
        {
            type Base = T;

            fn validate(value: Self::Base) -> bool {
                C1::validate(value) && C2::validate(value) && C3::validate(value)
            }
        }
    }
}

/// 生成 Constrained 结构体和基本方法
fn generate_constrained_struct() -> TokenStream2 {
    quote! {
        /// 受约束的浮点数泛型结构
        #[derive(Clone, Copy)]
        pub struct Constrained<T, V> {
            value: T,
            phantom: std::marker::PhantomData<V>,
        }

        impl<T: std::fmt::Display + Copy, V: Constraint<Base = T>> Constrained<T, V> {
            /// 创建新的受约束浮点数
            ///
            /// # 示例
            ///
            /// ```
            /// use strict_num_extended::FinF32;
            ///
            /// let finite = FinF32::new(3.14);
            /// assert_eq!(finite.unwrap().get(), 3.14);
            /// ```
            ///
            /// 如果值不满足约束条件，返回 `None`。
            #[must_use]
            pub fn new(value: T) -> Option<Self> {
                V::validate(value).then_some(Self {
                    value,
                    phantom: std::marker::PhantomData,
                })
            }

            /// 不安全地创建受约束浮点数（不进行验证）
            ///
            /// # Safety
            ///
            /// 调用者必须确保值满足约束条件。
            /// 违反约束会导致未定义行为。
            #[inline]
            pub const unsafe fn new_unchecked(value: T) -> Self {
                Self {
                    value,
                    phantom: std::marker::PhantomData,
                }
            }

            /// 获取内部值
            ///
            /// # 示例
            ///
            /// ```
            /// use strict_num_extended::FinF32;
            ///
            /// let finite = FinF32::new(2.5);
            /// assert_eq!(finite.unwrap().get(), 2.5);
            /// ```
            #[must_use]
            pub const fn get(&self) -> T {
                self.value
            }

            /// 尝试从另一个类型转换
            ///
            /// # 示例
            ///
            /// ```
            /// use strict_num_extended::FinF32;
            ///
            /// let value = 3.14f32;
            /// let finite_32 = FinF32::try_from(value);
            /// assert!(finite_32.is_ok());
            /// ```
            ///
            /// # Errors
            ///
            /// 如果转换后的值不满足约束条件，返回 `Err(())`。
            #[must_use = "返回值可能包含错误，不应被忽略"]
            #[expect(clippy::result_unit_err)]
            pub fn try_from<U>(value: U) -> Result<Self, ()>
            where
                U: std::fmt::Display + Copy,
                T: From<U>,
                V: Constraint<Base = T>,
            {
                Self::new(T::from(value)).ok_or(())
            }
        }
    }
}

/// 生成比较和格式化 trait 实现
fn generate_comparison_traits() -> TokenStream2 {
    quote! {
        use std::cmp::Ordering;
        use std::fmt;
        use std::ops::{Add, Sub, Mul, Div};

        // 比较运算实现
        impl<T: PartialEq, V> PartialEq for Constrained<T, V> {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl<T: PartialEq, V> Eq for Constrained<T, V> {}

        impl<T: PartialOrd, V> Ord for Constrained<T, V> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.value
                    .partial_cmp(&other.value)
                    .expect("Constrained values should always be comparable")
            }
        }

        impl<T: PartialOrd, V> PartialOrd for Constrained<T, V> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // 格式化实现
        impl<T: fmt::Display, V> fmt::Display for Constrained<T, V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl<T: fmt::Debug, V> fmt::Debug for Constrained<T, V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Constrained({:?})", self.value)
            }
        }
    }
}

/// 生成算术运算实现
fn generate_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    let traits_to_impl: Vec<_> = config
        .features
        .impl_traits
        .iter()
        .filter_map(|t| match t.to_string().as_str() {
            "Add" => Some(("Add", "add", quote! { + })),
            "Sub" => Some(("Sub", "sub", quote! { - })),
            "Mul" => Some(("Mul", "mul", quote! { * })),
            "Div" => Some(("Div", "div", quote! { / })),
            _ => None,
        })
        .collect();

    for (trait_name, method_name, op) in traits_to_impl {
        let trait_ident = Ident::new(trait_name, Span::call_site());
        let method_ident = Ident::new(method_name, Span::call_site());

        impls.push(quote! {
            impl<T, V> #trait_ident for Constrained<T, V>
            where
                T: std::fmt::Display + Copy + #trait_ident<Output = T>,
                V: Constraint<Base = T>,
            {
                type Output = Self;

                fn #method_ident(self, rhs: Self) -> Self::Output {
                    let result = self.value #op rhs.value;
                    Self::new(result).expect(concat!(
                        "Arithmetic operation failed: ",
                        stringify!(#trait_name)
                    ))
                }
            }
        });
    }

    quote! {
        #(#impls)*
    }
}

/// 生成类型别名
fn generate_type_aliases(config: &TypeConfig) -> TokenStream2 {
    let mut aliases = Vec::new();
    let mut option_aliases = Vec::new();

    for type_def in &config.constraint_types {
        match type_def {
            TypeDef::Single {
                type_name,
                float_types,
                constraint_name,
            } => {
                // 单约束类型别名
                for float_type in float_types {
                    let alias_name = format_ident!("{}{}", type_name, to_uppercase(float_type));

                    aliases.push(quote! {
                        #[doc = concat!(
                            stringify!(#type_name), " 约束的 ", stringify!(#float_type), " 值"
                        )]
                        pub type #alias_name = Constrained<#float_type, #constraint_name<#float_type>>;
                    });
                }
            }
            TypeDef::Combined {
                type_name,
                float_types,
                constraints,
            } => {
                // 组合约束类型别名
                for float_type in float_types {
                    let alias_name = format_ident!("{}{}", type_name, to_uppercase(float_type));

                    // 构建约束元组
                    let constraint_types: Vec<_> = constraints
                        .iter()
                        .map(|c| quote! { #c<#float_type> })
                        .collect();

                    aliases.push(quote! {
                        #[doc = concat!(
                            stringify!(#type_name), " 约束的 ", stringify!(#float_type), " 值"
                        )]
                        pub type #alias_name = Constrained<#float_type, (#(#constraint_types),*)>;
                    });
                }
            }
        }
    }

    // Option 类型别名
    if config.features.generate_option_types {
        for type_def in &config.constraint_types {
            match type_def {
                TypeDef::Single {
                    type_name,
                    float_types,
                    ..
                } => {
                    for float_type in float_types {
                        let type_alias = format_ident!("{}{}", type_name, to_uppercase(float_type));
                        let opt_alias = format_ident!("Opt{}", type_alias);

                        option_aliases.push(quote! {
                            #[doc = concat!("`", stringify!(#type_alias), "` 的 Option 版本")]
                            pub type #opt_alias = Option<#type_alias>;
                        });
                    }
                }
                TypeDef::Combined {
                    type_name,
                    float_types,
                    ..
                } => {
                    for float_type in float_types {
                        let type_alias = format_ident!("{}{}", type_name, to_uppercase(float_type));
                        let opt_alias = format_ident!("Opt{}", type_alias);

                        option_aliases.push(quote! {
                            #[doc = concat!("`", stringify!(#type_alias), "` 的 Option 版本")]
                            pub type #opt_alias = Option<#type_alias>;
                        });
                    }
                }
            }
        }
    }

    quote! {
        // 类型别名
        #(#aliases)*

        // Option 类型别名
        #(#option_aliases)*
    }
}

/// 生成 new_const 方法
fn generate_new_const_methods(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        match type_def {
            TypeDef::Single {
                type_name,
                float_types,
                constraint_name,
            } => {
                // 为单约束类型生成
                let constraint_def = config
                    .constraints
                    .iter()
                    .find(|c| &c.name == constraint_name)
                    .expect("Constraint definition not found");

                let validate: Expr =
                    syn::parse_str(&constraint_def.validate).unwrap_or_else(|_| {
                        panic!("Invalid validate expression: {}", constraint_def.validate)
                    });

                for float_type in float_types {
                    let type_alias = format_ident!("{}{}", type_name, to_uppercase(float_type));

                    impls.push(quote! {
                        impl #type_alias {
                            /// 在编译期创建值
                            ///
                            /// # Panics
                            ///
                            /// 如果值不满足约束条件，在编译期或运行时会 panic。
                            #[inline]
                            #[must_use]
                            pub const fn new_const(value: #float_type) -> Self {
                                if #validate {
                                    unsafe { Self::new_unchecked(value) }
                                } else {
                                    panic!("Value does not satisfy the constraint");
                                }
                            }
                        }
                    });
                }
            }
            TypeDef::Combined {
                type_name,
                float_types,
                constraints,
            } => {
                // 为组合约束类型生成
                for float_type in float_types {
                    let type_alias = format_ident!("{}{}", type_name, to_uppercase(float_type));

                    // 收集所有约束的验证条件
                    let mut checks = Vec::new();
                    for constraint_name in constraints {
                        let constraint_def = config
                            .constraints
                            .iter()
                            .find(|c| &c.name == constraint_name)
                            .expect("Constraint definition not found");

                        let validate: Expr = syn::parse_str(&constraint_def.validate)
                            .unwrap_or_else(|_| {
                                panic!("Invalid validate expression: {}", constraint_def.validate)
                            });
                        checks.push(validate);
                    }

                    // 组合所有检查条件
                    let combined_check = checks
                        .iter()
                        .fold(quote! { true }, |acc, check| quote! { #acc && #check });

                    impls.push(quote! {
                        impl #type_alias {
                            /// 在编译期创建值
                            ///
                            /// # Panics
                            ///
                            /// 如果值不满足约束条件，在编译期或运行时会 panic。
                            #[inline]
                            #[must_use]
                            pub const fn new_const(value: #float_type) -> Self {
                                if #combined_check {
                                    unsafe { Self::new_unchecked(value) }
                                } else {
                                    panic!("Value does not satisfy the constraint");
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    quote! {
        #(#impls)*
    }
}

/// 将 f32/f64 转换为 F32/F64
fn to_uppercase(ident: &Ident) -> String {
    ident.to_string().to_uppercase()
}
