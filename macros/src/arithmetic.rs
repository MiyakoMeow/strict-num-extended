//! Arithmetic operations module

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};

use crate::config::{ArithmeticOp, ArithmeticResult, TypeConfig};

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Generates arithmetic operations for given ops using a generator function.
fn generate_arithmetic_for_ops<F>(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
    mut impl_generator: F,
) -> TokenStream2
where
    F: FnMut(
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        TokenStream2,
        &ArithmeticResult,
        ArithmeticOp,
    ) -> TokenStream2,
{
    let mut impls = Vec::new();

    for lhs_type in &config.constraint_types {
        for rhs_type in &config.constraint_types {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Get the arithmetic result from the precomputed table
                let key = (
                    *op,
                    lhs_type.type_name.to_string(),
                    rhs_type.type_name.to_string(),
                );
                let result = config
                    .arithmetic_results
                    .get(&key)
                    .expect("Arithmetic result not found");

                for float_type in &lhs_type.float_types {
                    let lhs_alias = make_type_alias(&lhs_type.type_name, float_type);
                    let rhs_alias = make_type_alias(&rhs_type.type_name, float_type);
                    let output_alias = make_type_alias(&result.output_type, float_type);

                    let impl_code = impl_generator(
                        lhs_alias,
                        rhs_alias,
                        output_alias,
                        trait_ident.clone(),
                        method_ident.clone(),
                        op_symbol.clone(),
                        result,
                        *op,
                    );

                    impls.push(impl_code);
                }
            }
        }
    }

    quote! {
        #(#impls)*
    }
}

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

    generate_arithmetic_for_ops(
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
                            // SAFETY: The operation is proven safe by type constraints
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

/// Generates arithmetic operations for Option types.
///
/// Due to orphan rules, we can only implement:
/// - `Lhs op Option<Rhs>` -> Option<Output> or Result<Option<Output>, `FloatError`>
///
/// For `Option<Lhs> op Rhs` and `Option<Lhs> op Option<Rhs>`, users need to use
/// `.map()` or pattern matching since we can't implement traits for `Option<T>`.
pub fn generate_option_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let ops = [
        (ArithmeticOp::Add, "Add", "add", quote! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", quote! { * }),
        (ArithmeticOp::Div, "Div", "div", quote! { / }),
    ];

    generate_arithmetic_for_ops(
        config,
        &ops,
        |lhs_alias, rhs_alias, output_alias, trait_ident, method_ident, _op_symbol, result, _op| {
            // Check if the operation is safe (returns direct value) or fallible (returns Result)
            if result.is_safe {
                // Safe operation: returns direct type, so Option operation returns Option<Output>
                quote! {
                    impl #trait_ident<Option<#rhs_alias>> for #lhs_alias {
                        type Output = Option<#output_alias>;

                        fn #method_ident(self, rhs: Option<#rhs_alias>) -> Self::Output {
                            match rhs {
                                Some(b) => {
                                    let inner_result = self.#method_ident(b);
                                    Some(inner_result)
                                }
                                None => None,
                            }
                        }
                    }
                }
            } else {
                // Fallible operation: returns Result, so Option operation returns Result<Option<Output>, FloatError>
                quote! {
                    impl #trait_ident<Option<#rhs_alias>> for #lhs_alias {
                        type Output = Result<Option<#output_alias>, FloatError>;

                        fn #method_ident(self, rhs: Option<#rhs_alias>) -> Self::Output {
                            match rhs {
                                Some(b) => {
                                    self.#method_ident(b).map(Some)
                                }
                                None => Ok(None),
                            }
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

        // Find the constraint definition
        let Some(constraint_def) = config
            .constraints
            .iter()
            .find(|c| c.name == type_def.constraint_name)
        else {
            continue;
        };

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
                        #neg_type_alias::new(result)
                            .expect("Negation operation failed: result does not satisfy constraint")
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
}
