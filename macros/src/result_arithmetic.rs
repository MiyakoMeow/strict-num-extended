//! Result type arithmetic operations module
//!
//! Generates arithmetic operations for Result<T, `FloatError`> types,
//! supporting three combination patterns:
//! 1. Lhs op Result<Rhs, `FloatError`>
//! 2. Result<Lhs, `FloatError`> op Rhs
//! 3. Result<Lhs, `FloatError`> op Result<Rhs, `FloatError`>

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};

use crate::config::{ArithmeticOp, ArithmeticResult, TypeConfig};

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Generates arithmetic operations for given ops using a generator function.
fn generate_result_arithmetic_for_ops<F>(
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
                        lhs_alias.clone(),
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

/// Generates arithmetic operations for Result types.
///
/// Supports two patterns (pattern 3 violates orphan rule):
/// 1. `Lhs op Result<Rhs, FloatError>` -> Result<Output, FloatError>
/// 2. `Result<Lhs, FloatError> op Rhs` -> Result<Output, FloatError>
///
/// Error propagation strategy:
/// - Safe operations: wrap concrete result in Ok(...)
/// - Fallible operations: directly propagate Result from base operation
/// - Division: zero check is handled by base operation
pub fn generate_result_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let ops = [
        (ArithmeticOp::Add, "Add", "add", quote! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", quote! { * }),
        (ArithmeticOp::Div, "Div", "div", quote! { / }),
    ];

    // Generate implementations for all three patterns
    let pattern1_impls = generate_pattern_lhs_op_result_rhs(config, &ops);
    let pattern2_impls = generate_pattern_result_lhs_op_rhs(config, &ops);
    // let pattern3_impls = generate_pattern_result_lhs_op_result_rhs(config, &ops);

    quote! {
        #pattern1_impls
        #pattern2_impls
        // #pattern3_impls
    }
}

/// Pattern 1: Lhs op Result<Rhs, `FloatError`>
fn generate_pattern_lhs_op_result_rhs(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
) -> TokenStream2 {
    generate_result_arithmetic_for_ops(
        config,
        ops,
        |lhs_alias, rhs_alias, output_alias, trait_ident, method_ident, _op_symbol, result, _op| {
            if result.is_safe {
                // Safe operation: base returns concrete type, wrap in Ok
                quote! {
                    impl #trait_ident<Result<#rhs_alias, FloatError>> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: Result<#rhs_alias, FloatError>) -> Self::Output {
                            match rhs {
                                Ok(b) => Ok(self.#method_ident(b)),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            } else {
                // Fallible operation: base returns Result, directly propagate
                quote! {
                    impl #trait_ident<Result<#rhs_alias, FloatError>> for #lhs_alias {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: Result<#rhs_alias, FloatError>) -> Self::Output {
                            match rhs {
                                Ok(b) => self.#method_ident(b),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Pattern 2: Result<Lhs, `FloatError`> op Rhs
fn generate_pattern_result_lhs_op_rhs(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
) -> TokenStream2 {
    generate_result_arithmetic_for_ops(
        config,
        ops,
        |lhs_alias, rhs_alias, output_alias, trait_ident, method_ident, _op_symbol, result, _op| {
            if result.is_safe {
                // Safe operation: base returns concrete type, wrap in Ok
                quote! {
                    impl #trait_ident<#rhs_alias> for Result<#lhs_alias, FloatError> {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            match self {
                                Ok(a) => Ok(a.#method_ident(rhs)),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            } else {
                // Fallible operation: base returns Result, directly propagate
                quote! {
                    impl #trait_ident<#rhs_alias> for Result<#lhs_alias, FloatError> {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            match self {
                                Ok(a) => a.#method_ident(rhs),
                                Err(e) => Err(e),
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Pattern 3: Result<Lhs, `FloatError`> op Result<Rhs, `FloatError`>
fn generate_pattern_result_lhs_op_result_rhs(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
) -> TokenStream2 {
    generate_result_arithmetic_for_ops(
        config,
        ops,
        |lhs_alias, rhs_alias, output_alias, trait_ident, method_ident, _op_symbol, result, _op| {
            if result.is_safe {
                // Safe operation: base returns concrete type, wrap in Ok
                quote! {
                    impl #trait_ident<Result<#rhs_alias, FloatError>> for Result<#lhs_alias, FloatError> {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: Result<#rhs_alias, FloatError>) -> Self::Output {
                            match (self, rhs) {
                                (Ok(a), Ok(b)) => Ok(a.#method_ident(b)),
                                (Err(e), _) | (_, Err(e)) => Err(e),
                            }
                        }
                    }
                }
            } else {
                // Fallible operation: base returns Result, directly propagate
                quote! {
                    impl #trait_ident<Result<#rhs_alias, FloatError>> for Result<#lhs_alias, FloatError> {
                        type Output = Result<#output_alias, FloatError>;

                        fn #method_ident(self, rhs: Result<#rhs_alias, FloatError>) -> Self::Output {
                            match (self, rhs) {
                                (Ok(a), Ok(b)) => a.#method_ident(b),
                                (Err(e), _) | (_, Err(e)) => Err(e),
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Generates unary negation operation implementations for Result types.
///
/// Generates `impl Neg for Result<T, FloatError>` for all types that support negation.
pub fn generate_result_neg_impls(config: &TypeConfig) -> TokenStream2 {
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
                impl Neg for Result<#type_alias, FloatError> {
                    type Output = Result<#neg_type_alias, FloatError>;

                    fn neg(self) -> Self::Output {
                        self.map(|a| -a)
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
}
