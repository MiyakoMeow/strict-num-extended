//! # `FromStr` trait implementation generation module
//!
//! Automatically generates `FromStr` implementations for all constraint types

use crate::config::TypeConfig;
use crate::generator::for_all_constraint_float_types;
use quote::quote;

/// Generate `FloatParseError` type and its trait implementations
pub fn generate_parse_error_type() -> proc_macro2::TokenStream {
    quote! {
        /// String parsing error
        ///
        /// Contains two possible error variants:
        /// 1. String cannot be parsed as a floating-point number
        /// 2. Parsing succeeded but validation failed (wraps FloatError)
        #[derive(Debug, Clone, PartialEq, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum FloatParseError {
            /// String cannot be parsed as a valid floating-point number
            InvalidFloat {
                /// Original input string
                input: String,
            },
            /// Parsing succeeded but value validation failed
            ValidationFailed(FloatError),
        }

        impl core::fmt::Display for FloatParseError {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    FloatParseError::InvalidFloat { input } => {
                        write!(f, "failed to parse '{}' as a floating-point number", input)
                    }
                    FloatParseError::ValidationFailed(e) => write!(f, "{}", e),
                }
            }
        }

        #[cfg(feature = "std")]
        impl std::error::Error for FloatParseError {}
    }
}

/// Generate all `FromStr` implementations
pub fn generate_fromstr_traits(config: &TypeConfig) -> proc_macro2::TokenStream {
    let impls = for_all_constraint_float_types(config, |type_name, float_type, _| {
        let struct_name = crate::generator::make_type_alias(type_name, float_type);

        quote! {
            impl core::str::FromStr for #struct_name {
                type Err = FloatParseError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    // 1. Parse using standard library (automatically supports scientific notation)
                    let value = #float_type::from_str(s).map_err(|_| FloatParseError::InvalidFloat {
                        input: s.to_string(),
                    })?;

                    // 2. Validate constraints (reuses existing new() logic)
                    Self::new(value).map_err(FloatParseError::ValidationFailed)
                }
            }
        }
    });

    quote! { #(#impls)* }
}
