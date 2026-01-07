//! Arithmetic operations module

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::config::{ArithmeticOp, TypeConfig};
use crate::generator::{find_constraint_def, generate_arithmetic_for_all_types, make_type_alias};

/// Generates type-safe arithmetic operation implementations.
///
/// This generates cross-type arithmetic operations with automatic output type inference.
/// Safe operations return the result directly, while potentially failing operations return Option.
pub fn generate_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let ops = [
        (ArithmeticOp::Add, "Add", "add", quote! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", quote! { * }),
        (ArithmeticOp::Div, "Div", "div", quote! { / }),
    ];

    generate_arithmetic_for_all_types(
        config,
        &ops,
        |lhs_alias, rhs_alias, output_alias, trait_ident, method_ident, op_symbol, result, op| {
            if result.is_safe {
                // Safe operation: return result directly
                quote! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = #output_alias;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            let result = self.get() #op_symbol rhs.get();
                            // SAFETY: The arithmetic configuration has proven at compile time that
                            // for this specific combination of lhs_type and rhs_type, the result
                            // is guaranteed to satisfy output_type's constraints. The validation
                            // would be redundant since the type system already guarantees safety.
                            unsafe { #output_alias::new_unchecked(result) }
                        }
                    }
                }
            } else if op == ArithmeticOp::Div {
                // Division: always check for zero
                quote! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            let rhs_val = rhs.get();
                            if rhs_val == 0.0 {
                                return Err(FloatError::DivisionByZero);
                            }
                            let result = self.get() / rhs_val;
                            #output_alias::new(result)
                        }
                    }
                }
            } else {
                // Potentially failing operation: return Result
                quote! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            let result = self.get() #op_symbol rhs.get();
                            #output_alias::new(result)
                        }
                    }
                }
            }
        },
    )
}

/// Generates unary negation operation implementations.
pub fn generate_neg_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;

        // 使用辅助函数查找约束定义
        let constraint_def = find_constraint_def(config, &type_def.constraint_name);

        // Skip if no corresponding negation type
        let Some(neg_constraint_name) = &constraint_def.neg_constraint_name else {
            continue;
        };

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let neg_type_alias = make_type_alias(neg_constraint_name, float_type);

            impls.push(quote! {
                impl Neg for #type_alias {
                    type Output = #neg_type_alias;

                    fn neg(self) -> Self::Output {
                        let result = -self.get();
                        // SAFETY: The negation constraint was computed at compile time by
                        // negating the source constraint's conditions and finding a matching
                        // constraint. Since neg_constraint_name was found through condition
                        // matching, the result is mathematically guaranteed to satisfy the
                        // target constraint. The runtime validation would be redundant.
                        unsafe { #neg_type_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
}
