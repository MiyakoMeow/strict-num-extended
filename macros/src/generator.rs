//! Code generation module
//!
//! Contains all functions for generating Rust code.

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::Expr;

use crate::config::{TypeConfig, TypeDef};

// ============================================================================
// Helper functions
// ============================================================================

/// Converts floating-point type identifier to uppercase
fn to_uppercase_ident(ident: &Ident) -> String {
    ident.to_string().to_uppercase()
}

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, to_uppercase_ident(float_type))
}

/// Parses constraint validation expression
///
/// # Panics
/// Panics if the expression is invalid
fn parse_validate_expr(validate_str: &str, constraint_name: &str) -> Expr {
    syn::parse_str(validate_str).unwrap_or_else(|e| {
        panic!(
            "Invalid validate expression for constraint '{}': {}\nExpression: {}",
            constraint_name, e, validate_str
        );
    })
}

// ============================================================================
// Code generation functions
// ============================================================================

/// Generates Constraint trait.
pub fn generate_constraint_trait() -> TokenStream2 {
    quote! {
        /// Constraint type marker trait
        pub trait Constraint {
            /// Base type (f32 or f64)
            type Base;

            /// Validates whether a value satisfies the constraint
            ///
            /// Returns `true` if the value satisfies the constraint, `false` otherwise.
            fn validate(value: Self::Base) -> bool;
        }
    }
}

/// Generates constraint marker types.
pub fn generate_constraint_markers(config: &TypeConfig) -> TokenStream2 {
    let mut markers = Vec::new();
    let mut impls = Vec::new();

    for constraint in &config.constraints {
        let name = &constraint.name;
        let doc = &constraint.doc;

        // Parse validation expression
        let validate = parse_validate_expr(&constraint.validate, &constraint.name.to_string());

        // Generate marker types
        markers.push(quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy)]
            pub struct #name<F = ()> {
                _marker: std::marker::PhantomData<F>,
            }
        });

        // Generate f32 implementation
        impls.push(quote! {
            impl Constraint for #name<f32> {
                type Base = f32;

                fn validate(value: Self::Base) -> bool {
                    #validate
                }
            }
        });

        // Generate f64 implementation
        impls.push(quote! {
            impl Constraint for #name<f64> {
                type Base = f64;

                fn validate(value: Self::Base) -> bool {
                    #validate
                }
            }
        });
    }

    // Generate tuple combination constraint implementations
    let tuple_impls = generate_tuple_constraints();

    quote! {
        // Constraint marker types
        #(#markers)*

        // Constraint trait implementations
        #(#impls)*

        // Tuple combination constraints
        #tuple_impls
    }
}

/// Generates tuple combination constraints.
pub fn generate_tuple_constraints() -> TokenStream2 {
    quote! {
        /// Single-element tuple (C1,)
        impl<T, C1> Constraint for (C1,)
        where
            T: Copy,
            C1: Constraint<Base = T>,
        {
            type Base = T;

            fn validate(value: Self::Base) -> bool {
                C1::validate(value)
            }
        }

        /// Two-element tuple (C1, C2)
        impl<T, C1, C2> Constraint for (C1, C2)
        where
            T: Copy,
            C1: Constraint<Base = T>,
            C2: Constraint<Base = T>,
        {
            type Base = T;

            fn validate(value: Self::Base) -> bool {
                C1::validate(value) && C2::validate(value)
            }
        }

        /// Three-element tuple (C1, C2, C3)
        impl<T, C1, C2, C3> Constraint for (C1, C2, C3)
        where
            T: Copy,
            C1: Constraint<Base = T>,
            C2: Constraint<Base = T>,
            C3: Constraint<Base = T>,
        {
            type Base = T;

            fn validate(value: Self::Base) -> bool {
                C1::validate(value) && C2::validate(value) && C3::validate(value)
            }
        }
    }
}

/// Generates Constrained struct and basic methods.
pub fn generate_constrained_struct() -> TokenStream2 {
    quote! {
        /// Generic constrained floating-point structure
        #[derive(Clone, Copy)]
        pub struct Constrained<T, V> {
            value: T,
            phantom: std::marker::PhantomData<V>,
        }

        impl<T: std::fmt::Display + Copy, V: Constraint<Base = T>> Constrained<T, V> {
            /// Creates a new constrained floating-point number
            ///
            /// # Example
            ///
            /// ```
            /// use strict_num_extended::FinF32;
            ///
            /// let finite = FinF32::new(3.14);
            /// assert_eq!(finite.unwrap().get(), 3.14);
            /// ```
            ///
            /// Returns `None` if the value does not satisfy the constraint.
            #[must_use]
            pub fn new(value: T) -> Option<Self> {
                V::validate(value).then_some(Self {
                    value,
                    phantom: std::marker::PhantomData,
                })
            }

            /// Unsafely creates a constrained floating-point number (no validation)
            ///
            /// # Safety
            ///
            /// Caller must ensure the value satisfies the constraint.
            /// Violating the constraint leads to undefined behavior.
            #[inline]
            pub const unsafe fn new_unchecked(value: T) -> Self {
                Self {
                    value,
                    phantom: std::marker::PhantomData,
                }
            }

            /// Gets the inner value
            ///
            /// # Example
            ///
            /// ```
            /// use strict_num_extended::FinF32;
            ///
            /// let finite = FinF32::new(2.5);
            /// assert_eq!(finite.unwrap().get(), 2.5);
            /// ```
            #[must_use]
            pub const fn get(&self) -> T {
                self.value
            }

            /// Attempts to convert from another type
            ///
            /// # Example
            ///
            /// ```
            /// use strict_num_extended::FinF32;
            ///
            /// let value = 3.14f32;
            /// let finite_32 = FinF32::try_from(value);
            /// assert!(finite_32.is_ok());
            /// ```
            ///
            /// # Errors
            ///
            /// Returns `Err(())` if the converted value does not satisfy the constraint.
            #[must_use = "Return value may contain an error and should not be ignored"]
            #[expect(clippy::result_unit_err)]
            pub fn try_from<U>(value: U) -> Result<Self, ()>
            where
                U: std::fmt::Display + Copy,
                T: From<U>,
                V: Constraint<Base = T>,
            {
                Self::new(T::from(value)).ok_or(())
            }
        }
    }
}

/// Generates comparison and formatting trait implementations.
pub fn generate_comparison_traits() -> TokenStream2 {
    quote! {
        use std::cmp::Ordering;
        use std::fmt;
        use std::ops::{Add, Sub, Mul, Div};

        // Comparison operation implementations
        impl<T: PartialEq, V> PartialEq for Constrained<T, V> {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl<T: PartialEq, V> Eq for Constrained<T, V> {}

        impl<T: PartialOrd, V> Ord for Constrained<T, V> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.value
                    .partial_cmp(&other.value)
                    .expect("Constrained values should always be comparable")
            }
        }

        impl<T: PartialOrd, V> PartialOrd for Constrained<T, V> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // Formatting implementations
        impl<T: fmt::Display, V> fmt::Display for Constrained<T, V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl<T: fmt::Debug, V> fmt::Debug for Constrained<T, V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Constrained({:?})", self.value)
            }
        }
    }
}

/// Generates arithmetic operation implementations.
pub fn generate_arithmetic_impls(_config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    // Always implement all arithmetic operations
    let traits_to_impl: &[(&str, &str, TokenStream2)] = &[
        ("Add", "add", quote! { + }),
        ("Sub", "sub", quote! { - }),
        ("Mul", "mul", quote! { * }),
        ("Div", "div", quote! { / }),
    ];

    for (trait_name, method_name, op) in traits_to_impl {
        let trait_ident = Ident::new(trait_name, Span::call_site());
        let method_ident = Ident::new(method_name, Span::call_site());

        impls.push(quote! {
            impl<T, V> #trait_ident for Constrained<T, V>
            where
                T: std::fmt::Display + Copy + #trait_ident<Output = T>,
                V: Constraint<Base = T>,
            {
                type Output = Self;

                fn #method_ident(self, rhs: Self) -> Self::Output {
                    let result = self.value #op rhs.value;
                    Self::new(result).expect(concat!(
                        "Arithmetic operation failed: ",
                        stringify!(#trait_name)
                    ))
                }
            }
        });
    }

    quote! {
        #(#impls)*
    }
}

/// Generates type aliases.
pub fn generate_type_aliases(config: &TypeConfig) -> TokenStream2 {
    let mut aliases = Vec::new();
    let mut option_aliases = Vec::new();

    for type_def in &config.constraint_types {
        match type_def {
            TypeDef::Single {
                type_name,
                float_types,
                constraint_name,
            } => {
                // Single constraint type aliases
                for float_type in float_types {
                    let alias_name = make_type_alias(type_name, float_type);

                    aliases.push(quote! {
                        #[doc = concat!(
                            stringify!(#type_name), " constrained ", stringify!(#float_type), " value"
                        )]
                        pub type #alias_name = Constrained<#float_type, #constraint_name<#float_type>>;
                    });
                }
            }
        }
    }

    // Option type aliases (always generate)
    for type_def in &config.constraint_types {
        // Use TypeDef helper methods to get type name and floating-point types
        let type_name = type_def.type_name();
        let float_types = type_def.float_types();

        for float_type in float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let opt_alias = format_ident!("Opt{}", type_alias);

            option_aliases.push(quote! {
                #[doc = concat!("`", stringify!(#type_alias), "` Option version")]
                pub type #opt_alias = Option<#type_alias>;
            });
        }
    }

    quote! {
        // Type aliases
        #(#aliases)*

        // Option type aliases
        #(#option_aliases)*
    }
}

/// Generates `new_const` methods.
pub fn generate_new_const_methods(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        match type_def {
            TypeDef::Single {
                type_name,
                float_types,
                constraint_name,
            } => {
                // Generate for single constraint types
                let constraint_def = config
                    .constraints
                    .iter()
                    .find(|c| &c.name == constraint_name)
                    .expect("Constraint definition not found");

                let validate =
                    parse_validate_expr(&constraint_def.validate, &constraint_def.name.to_string());

                for float_type in float_types {
                    let type_alias = make_type_alias(type_name, float_type);

                    impls.push(quote! {
                        impl #type_alias {
                            /// Creates a value at compile time
                            ///
                            /// # Panics
                            ///
                            /// Will [`panic`] at compile time or runtime if the value does not satisfy the constraint.
                            #[inline]
                            #[must_use]
                            pub const fn new_const(value: #float_type) -> Self {
                                if #validate {
                                    unsafe { Self::new_unchecked(value) }
                                } else {
                                    panic!("Value does not satisfy the constraint");
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    quote! {
        #(#impls)*
    }
}
