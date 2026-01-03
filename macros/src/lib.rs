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
    generate_arithmetic_impls, generate_comparison_traits, generate_constrained_struct,
    generate_constraint_markers, generate_constraint_trait, generate_new_const_methods,
    generate_type_aliases,
};

/// Main macro: generates all code based on configuration.
#[proc_macro]
pub fn generate_constrained_types(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as TypeConfig);

    // Collect all code to be generated
    let mut all_code = vec![
        generate_constraint_trait(),
        generate_constraint_markers(&config),
        generate_constrained_struct(),
        generate_comparison_traits(),
    ];

    // Generate arithmetic operations
    if config
        .features
        .impl_traits
        .iter()
        .any(|t| matches!(t.to_string().as_str(), "Add" | "Sub" | "Mul" | "Div"))
    {
        all_code.push(generate_arithmetic_impls(&config));
    }

    // Generate type aliases
    all_code.push(generate_type_aliases(&config));

    // Generate new_const methods
    if config.features.generate_new_const {
        all_code.push(generate_new_const_methods(&config));
    }

    // Combine all code
    let expanded = quote! {
        #(#all_code)*
    };

    TokenStream::from(expanded)
}
