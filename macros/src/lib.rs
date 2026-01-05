//! # Proc Macro Implementation
//!
//! Provides complete procedural macro code generation for strict-num-extended

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod config;
mod generator;

use config::TypeConfig;
use generator::{
    generate_arithmetic_impls, generate_comparison_traits, generate_constraint_markers,
    generate_constraint_trait, generate_finite_float_struct, generate_neg_impls,
    generate_new_const_methods, generate_option_arithmetic_impls, generate_type_aliases,
};

/// Main macro: generates finite floating-point types with automatic `is_finite()` checking.
#[proc_macro]
pub fn generate_finite_float_types(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TypeConfig);

    // Collect all code to be generated
    let mut all_code = vec![
        generate_constraint_trait(),
        generate_constraint_markers(&config),
        generate_finite_float_struct(),
        generate_comparison_traits(),
    ];

    // Generate type-safe arithmetic operations
    all_code.push(generate_arithmetic_impls(&config));

    // Generate arithmetic operations for Option types
    all_code.push(generate_option_arithmetic_impls(&config));

    // Generate negation operations
    all_code.push(generate_neg_impls(&config));

    // Generate type aliases
    all_code.push(generate_type_aliases(&config));

    // Always generate new_const methods
    all_code.push(generate_new_const_methods(&config));

    // Combine all code
    let expanded = quote! {
        #(#all_code)*
    };

    TokenStream::from(expanded)
}
