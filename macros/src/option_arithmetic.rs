//! Option type arithmetic operations module
//!
//! Generates arithmetic operations for `Option<T>` types, supporting `Lhs op Option<Rhs>` pattern.
//!
//! # Orphan Rule Limitations
//!
//! Due to Rust's orphan rule, we cannot implement external traits for the external type `Option<T>`,
//! therefore the following patterns are **not implementable**:
//!
//! - `impl Add<Rhs> for Option<Lhs>` - `Option<Lhs> op Rhs`
//! - `impl Add<Option<Rhs>> for Option<Lhs>` - `Option<Lhs> op Option<Rhs>`
//! - `impl Neg for Option<T>` - `Option` negation operation
//!
//! # Feasible Pattern
//!
//! ✅ **Only feasible pattern**: `impl Add<Option<Rhs>> for Lhs` - `Lhs op Option<Rhs>`
//!
//! # Alternatives for Non-Implementable Patterns
//!
//! ## Negation Operation
//!
//! Use the `.map()` method:
//!
//! ```text
//! use strict_num_extended::*;
//!
//! let a: Option<PositiveF64> = Some(PositiveF64::new_const(5.0));
//! let neg: Option<NegativeF64> = a.map(|x| -x);
//! assert!(neg.is_some());
//! assert_eq!(neg.unwrap().get(), -5.0);
//!
//! // Handle None
//! let none: Option<PositiveF64> = None;
//! let neg_none: Option<NegativeF64> = none.map(|x| -x);
//! assert!(neg_none.is_none());
//! ```
//!
//! ## Option<Lhs> op Rhs
//!
//! Use `.map()` or `.and_then()`:
//!
//! ```text
//! let a: Option<PositiveF64> = Some(PositiveF64::new_const(5.0));
//! let b: NegativeF64 = NegativeF64::new_const(-3.0);
//!
//! // Use map
//! let result: Option<FinF64> = a.map(|a_val| a_val + b);
//! assert!(result.is_some());
//!
//! // Handle None
//! let none: Option<PositiveF64> = None;
//! let result_none: Option<FinF64> = none.map(|a_val| a_val + b);
//! assert!(result_none.is_none());
//! ```
//!
//! ## Option<Lhs> op Option<Rhs>
//!
//! Use pattern matching or combinators:
//!
//! ```text
//! let a: Option<PositiveF64> = Some(PositiveF64::new_const(5.0));
//! let b: Option<NegativeF64> = Some(NegativeF64::new_const(-3.0));
//!
//! // Use pattern matching
//! let result = match (a, b) {
//!     (Some(a_val), Some(b_val)) => Some(a_val + b_val),
//!     _ => None,
//! };
//! assert!(result.is_some());
//!
//! // Or use combinators from libraries like itertools
//! ```

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};

use crate::config::{ArithmeticOp, ArithmeticResult, TypeConfig};

/// Generates type alias identifier
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Generates Option arithmetic operation implementations for given operators
fn generate_option_arithmetic_for_ops<F>(
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

                // Get arithmetic result from precomputed table
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

/// Generates `Lhs op Option<Rhs>` pattern arithmetic operation implementations
fn generate_pattern_lhs_op_option_rhs(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
) -> TokenStream2 {
    generate_option_arithmetic_for_ops(
        config,
        ops,
        |lhs_alias, rhs_alias, output_alias, trait_ident, method_ident, _op_symbol, result, _op| {
            if result.is_safe {
                // Safe operation: base returns concrete type, so Option operation returns Option<Output>
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
                // Fallible operation: base returns Result, so Option operation returns Result<Option<Output>, FloatError>
                quote! {
                    impl #trait_ident<Option<#rhs_alias>> for #lhs_alias {
                        type Output = Result<Option<#output_alias>, FloatError>;

                        fn #method_ident(self, rhs: Option<#rhs_alias>) -> Self::Output {
                            match rhs {
                                Some(b) => self.#method_ident(b).map(Some),
                                None => Ok(None),
                            }
                        }
                    }
                }
            }
        },
    )
}

/// Generates Option type arithmetic operation implementations
///
/// # Supported Pattern
///
/// - `Lhs op Option<Rhs>` -> `Option<Output>` or `Result<Option<Output>, FloatError>`
///
/// # Return Type Rules
///
/// - **Safe operations** (e.g., `PositiveF64 + NegativeF64 -> FinF64`):
///   Returns `Option<Output>`
///
/// - **Fallible operations** (e.g., multiplication may cause overflow):
///   Returns `Result<Option<Output>, FloatError>`
///
/// - **Division operations**:
///   Returns `Result<Option<Output>, FloatError>` (includes zero check)
///
/// # Examples
///
/// ## Safe operations return Option
///
/// ```text
/// const A: PositiveF64 = PositiveF64::new_const(5.0);
/// let b: Option<NegativeF64> = Some(NegativeF64::new_const(-3.0));
/// let result: Option<FinF64> = A + b;
/// assert_eq!(result.unwrap().get(), 2.0);
/// ```
///
/// ## Handle None
///
/// ```text
/// const A: PositiveF64 = PositiveF64::new_const(5.0);
/// let b: Option<NegativeF64> = None;
/// let result: Option<FinF64> = A + b;
/// assert!(result.is_none());
/// ```
///
/// ## Fallible operations return Result
///
/// ```text
/// const A: PositiveF64 = PositiveF64::new_const(5.0);
/// let b: Option<PositiveF64> = Some(PositiveF64::new_const(3.0));
/// let result: Result<Option<PositiveF64>, FloatError> = A * b;
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap().unwrap().get(), 15.0);
/// ```
pub fn generate_option_arithmetic_impls(config: &TypeConfig) -> TokenStream2 {
    let ops = [
        (ArithmeticOp::Add, "Add", "add", quote! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", quote! { * }),
        (ArithmeticOp::Div, "Div", "div", quote! { / }),
    ];

    generate_pattern_lhs_op_option_rhs(config, &ops)
}
