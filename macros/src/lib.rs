//! # Proc Macro Implementation
//!
//! Provides complete procedural macro code generation for strict-num-extended

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod arithmetic;
mod comparison;
mod config;
mod finite_float;
mod float_conversion;
mod generator;
mod option_arithmetic;
mod result_arithmetic;
mod types;

use arithmetic::{generate_arithmetic_impls, generate_neg_impls};
use comparison::generate_comparison_traits;
use config::TypeConfig;
use finite_float::generate_finite_float_struct;
use float_conversion::{generate_as_f64_methods, generate_try_into_f32_methods};
use option_arithmetic::generate_option_arithmetic_impls;
use result_arithmetic::generate_result_arithmetic_impls;
use types::{generate_new_const_methods, generate_type_aliases};

/// Generates common definitions (Bounded struct and constants)
fn generate_common_definitions() -> proc_macro2::TokenStream {
    quote! {
        use std::marker::PhantomData;
        use std::ops::{Add, Sub, Mul, Div, Neg};

        // ========== f64 boundary bit representation constants ==========
        const F64_MIN_BITS: i64 = f64::MIN.to_bits() as i64;
        const F64_MAX_BITS: i64 = f64::MAX.to_bits() as i64;
        const ZERO_BITS: i64 = 0.0f64.to_bits() as i64;
        // Use minimum positive normal number instead of EPSILON (to avoid excluding very small positive numbers)
        const F64_MIN_POSITIVE_BITS: i64 = f64::MIN_POSITIVE.to_bits() as i64;
        const F64_NEG_MIN_POSITIVE_BITS: i64 = (-f64::MIN_POSITIVE).to_bits() as i64;
        const ONE_BITS: i64 = 1.0f64.to_bits() as i64;
        const NEG_ONE_BITS: i64 = (-1.0f64).to_bits() as i64;

        // ========== f32 boundary bit representation constants (stored as f64) ==========
        const F32_MIN_BITS: i64 = (f32::MIN as f64).to_bits() as i64;
        const F32_MAX_BITS: i64 = (f32::MAX as f64).to_bits() as i64;
        // Use minimum positive normal number instead of EPSILON
        const F32_MIN_POSITIVE_BITS: i64 = (f32::MIN_POSITIVE as f64).to_bits() as i64;
        const F32_NEG_MIN_POSITIVE_BITS: i64 = ((-f32::MIN_POSITIVE) as f64).to_bits() as i64;

        /// Boundary marker type (using i64 to encode f64 boundaries)
        #[derive(Debug, Clone, Copy)]
        pub struct Bounded<const MIN_BITS: i64, const MAX_BITS: i64, const EXCLUDE_ZERO: bool = false>;
    }
}

/// Generates the `FloatError` type and its trait implementations
fn generate_error_type() -> proc_macro2::TokenStream {
    quote! {
        /// Errors that can occur when creating or operating on finite floats
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum FloatError {
            /// Value is NaN (Not a Number)
            NaN,
            /// Value is positive infinity
            PosInf,
            /// Value is negative infinity
            NegInf,
            /// Value is outside the valid range for this type
            OutOfRange,
            /// Division by zero
            DivisionByZero,
        }

        impl std::fmt::Display for FloatError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    FloatError::NaN => write!(f, "value is NaN (Not a Number)"),
                    FloatError::PosInf => write!(f, "value is positive infinity"),
                    FloatError::NegInf => write!(f, "value is negative infinity"),
                    FloatError::OutOfRange => write!(f, "value is outside the valid range for this type"),
                    FloatError::DivisionByZero => write!(f, "division by zero"),
                }
            }
        }

        impl std::error::Error for FloatError {}
    }
}

/// Main macro: generates finite floating-point types with automatic `is_finite()` checking.
#[proc_macro]
pub fn generate_finite_float_types(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TypeConfig);

    // Collect all code to be generated
    let mut all_code = vec![
        generate_common_definitions(),
        generate_error_type(),
        generate_finite_float_struct(),
        generate_comparison_traits(),
    ];

    // Generate type aliases
    all_code.push(generate_type_aliases(&config));

    // Generate new_const methods
    all_code.push(generate_new_const_methods(&config));

    // Generate type-safe arithmetic operations
    all_code.push(generate_arithmetic_impls(&config));

    // Generate arithmetic operations for Option types
    all_code.push(generate_option_arithmetic_impls(&config));

    // Generate arithmetic operations for Result types
    all_code.push(generate_result_arithmetic_impls(&config));

    // Generate negation operations
    all_code.push(generate_neg_impls(&config));

    // Generate negation operations for Result types
    // Note: Cannot implement Neg for Result<T, E> due to orphan rules
    // Users should use .map() instead: result.map(|x| -x)
    // all_code.push(generate_result_neg_impls(&config));

    // Generate F32/F64 conversion methods (try_into_f32 for F64 types, as_f64 for F32 types)
    all_code.push(generate_try_into_f32_methods(&config));
    all_code.push(generate_as_f64_methods(&config));

    // Combine all code
    let expanded = quote! {
        #(#all_code)*
    };

    TokenStream::from(expanded)
}
