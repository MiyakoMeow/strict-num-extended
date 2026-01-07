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
