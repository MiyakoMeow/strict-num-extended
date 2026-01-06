//! Type aliases and constant construction methods module

use proc_macro2::Ident;
use quote::{format_ident, quote};

use crate::config::TypeConfig;
use crate::generator::{build_validation_expr, for_all_constraint_float_types};

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Generates type aliases.
pub fn generate_type_aliases(config: &TypeConfig) -> proc_macro2::TokenStream {
    // Generate regular type aliases
    let aliases = for_all_constraint_float_types(
        config,
        |type_name, float_type, constraint_def| {
            let alias_name = make_type_alias(type_name, float_type);

            // Calculate boundary constants from constraint bounds
            let min = constraint_def.bounds.lower.unwrap_or(f64::MIN);
            let max = constraint_def.bounds.upper.unwrap_or(f64::MAX);
            let min_bits = min.to_bits() as i64;
            let max_bits = max.to_bits() as i64;
            let exclude_zero = constraint_def.excludes_zero;

            quote! {
                #[doc = concat!(
                    stringify!(#type_name), " finite ", stringify!(#float_type), " value"
                )]
                pub type #alias_name = FiniteFloat<#float_type, Bounded<#min_bits, #max_bits, #exclude_zero>>;
            }
        },
    );

    // Generate Option type aliases
    let option_aliases =
        for_all_constraint_float_types(config, |type_name, float_type, _constraint_def| {
            let alias_name = make_type_alias(type_name, float_type);
            let opt_alias = format_ident!("Opt{}", alias_name);

            quote! {
                #[doc = concat!("`", stringify!(#alias_name), "` Option version")]
                pub type #opt_alias = Option<#alias_name>;
            }
        });

    quote! {
        // Type aliases
        #(#aliases)*

        // Option type aliases
        #(#option_aliases)*
    }
}

/// Generates `new_const` methods.
pub fn generate_new_const_methods(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, constraint_def| {
        let type_alias = make_type_alias(type_name, float_type);

        // Dynamically generate validation expression using constraint definition
        let validate_expr = build_validation_expr(constraint_def, float_type);

        quote! {
            impl #type_alias {
                /// Creates a value at compile time
                ///
                /// # Panics
                ///
                /// Will [`panic`] at compile time or runtime if the value does not satisfy the constraint.
                #[inline]
                #[must_use]
                pub const fn new_const(value: #float_type) -> Self {
                    if #validate_expr {
                        unsafe { Self::new_unchecked(value) }
                    } else {
                        panic!("Value does not satisfy the constraint");
                    }
                }
            }
        }
    });

    quote! {
        #(#impls)*
    }
}
