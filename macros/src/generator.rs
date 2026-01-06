//! Code generation module
//!
//! Contains all functions for generating Rust code.

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};

use crate::config::{ArithmeticOp, ArithmeticResult, ConstraintDef, TypeConfig};

// ============================================================================
// Helper functions
// ============================================================================

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Iterate over all constraint types and float types, generating code for each combination.
///
/// This function encapsulates the common pattern of iterating through all constraint types
/// and their associated float types, providing the constraint definition and type names
/// to a generator function.
///
/// # Arguments
///
/// * `config` - Type configuration containing constraint definitions
/// * `generator` - Function that generates code for each (`type_name`, `float_type`, `constraint_def`) combination
///
/// # Returns
///
/// A vector of generated token streams
fn for_all_constraint_float_types<F>(config: &TypeConfig, mut generator: F) -> Vec<TokenStream2>
where
    F: FnMut(&Ident, &Ident, &ConstraintDef) -> TokenStream2,
{
    let mut results = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;
        let constraint_def = config
            .constraints
            .iter()
            .find(|c| c.name == type_def.constraint_name)
            .expect("Constraint not found");

        for float_type in &type_def.float_types {
            results.push(generator(type_name, float_type, constraint_def));
        }
    }

    results
}

/// Dynamically builds validation expression based on constraint definition
fn build_validation_expr(constraint_def: &ConstraintDef, float_type: &Ident) -> TokenStream2 {
    let mut checks = Vec::new();

    // 1. Base check: is_finite()
    checks.push(quote! { value.is_finite() });

    // 2. Boundary checks
    if let Some(lower) = constraint_def.bounds.lower {
        let lower_check = build_bound_check(lower, true, constraint_def.excludes_zero, float_type);
        checks.push(lower_check);
    }

    if let Some(upper) = constraint_def.bounds.upper {
        let upper_check = build_bound_check(upper, false, constraint_def.excludes_zero, float_type);
        checks.push(upper_check);
    }

    // 3. Zero exclusion check (if not covered by bounds)
    if constraint_def.excludes_zero && needs_explicit_zero_check(constraint_def) {
        checks.push(quote! { value != 0.0 });
    }

    // Combine all checks with &&
    quote! {
        #(#checks)&&*
    }
}

/// Builds a single boundary check expression
fn build_bound_check(
    bound: f64,
    is_lower: bool,
    excludes_zero: bool,
    float_type: &Ident,
) -> TokenStream2 {
    let is_f32 = *float_type == "f32";

    // Determine whether to use strict comparison and whether to substitute with MIN_POSITIVE
    let (use_strict, use_min_positive) = match (is_lower, excludes_zero, bound == 0.0) {
        // Either bound, excludes zero, bound is zero -> use MIN_POSITIVE
        (_, true, true) => (false, true),
        // Otherwise use non-strict comparison without substitution
        _ => (false, false),
    };

    // Special handling for MIN_POSITIVE substitution
    let bound_value = if use_min_positive {
        // Use MIN_POSITIVE instead of 0.0 for strict comparison
        if is_f32 {
            quote! { (f32::MIN_POSITIVE as f64) }
        } else {
            quote! { f64::MIN_POSITIVE }
        }
    } else if is_f32 {
        // f32 needs conversion to f64
        quote! { (#bound as f64) }
    } else {
        quote! { #bound }
    };

    // f32 needs to convert value to f64 for comparison
    let value_expr = if is_f32 {
        quote! { (value as f64) }
    } else {
        quote! { value }
    };

    // Generate the appropriate comparison expression
    if is_lower {
        if use_strict {
            quote! { #value_expr > #bound_value }
        } else {
            quote! { #value_expr >= #bound_value }
        }
    } else if use_strict {
        quote! { #value_expr < #bound_value }
    } else {
        quote! { #value_expr <= #bound_value }
    }
}

/// Checks if an explicit zero check is needed
fn needs_explicit_zero_check(constraint_def: &ConstraintDef) -> bool {
    // If bounds already exclude zero through strict comparison (> or <), no need for explicit check
    let lower_excludes_zero =
        constraint_def.bounds.lower == Some(0.0) && constraint_def.excludes_zero;
    let upper_excludes_zero =
        constraint_def.bounds.upper == Some(0.0) && constraint_def.excludes_zero;

    // Need explicit check if bounds don't cover zero exclusion
    !lower_excludes_zero && !upper_excludes_zero
}

// ============================================================================
// Code generation functions
// ============================================================================

/// Generates `FloatBase` trait and constants.
pub fn generate_float_base_trait() -> TokenStream2 {
    quote! {
        /// Base trait for floating-point types, provides type conversion and validation methods
        pub trait FloatBase: Copy {
            /// Convert to f64 for boundary checking
            fn as_f64(self) -> f64;
            /// Check if the value is finite (not NaN, not infinity)
            fn is_finite(self) -> bool;
        }

        impl FloatBase for f32 {
            #[inline]
            fn as_f64(self) -> f64 {
                self as f64
            }

            #[inline]
            fn is_finite(self) -> bool {
                self.is_finite()
            }
        }

        impl FloatBase for f64 {
            #[inline]
            fn as_f64(self) -> f64 {
                self
            }

            #[inline]
            fn is_finite(self) -> bool {
                self.is_finite()
            }
        }

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

/// Generates `FiniteFloat` struct and basic methods.
pub fn generate_finite_float_struct() -> TokenStream2 {
    quote! {
        /// Generic finite floating-point structure
        #[derive(Clone, Copy)]
        pub struct FiniteFloat<T, B> {
            value: T,
            _marker: PhantomData<B>,
        }

        impl<T, const MIN_BITS: i64, const MAX_BITS: i64, const EXCLUDE_ZERO: bool>
            FiniteFloat<T, Bounded<MIN_BITS, MAX_BITS, EXCLUDE_ZERO>>
        where
            T: FloatBase,
        {
            /// Decodes boundary constants from bit representation
            const MIN: f64 = f64::from_bits(MIN_BITS as u64);
            const MAX: f64 = f64::from_bits(MAX_BITS as u64);

            /// Creates a new finite floating-point number
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
                let val_f64 = value.as_f64();

                let in_bounds = value.is_finite()
                    && val_f64 >= Self::MIN
                    && val_f64 <= Self::MAX;

                let not_zero = if EXCLUDE_ZERO {
                    (val_f64 as f64) != 0.0
                } else {
                    true
                };

                if in_bounds && not_zero {
                    Some(Self {
                        value,
                        _marker: PhantomData,
                    })
                } else {
                    None
                }
            }

            /// Unsafely creates a finite floating-point number (no validation)
            ///
            /// # Safety
            ///
            /// Caller must ensure the value satisfies the constraint.
            /// Violating the constraint leads to undefined behavior.
            #[inline]
            pub const unsafe fn new_unchecked(value: T) -> Self {
                Self {
                    value,
                    _marker: PhantomData,
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
                U: FloatBase,
                T: From<U>,
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

        // Comparison operation implementations
        impl<T: PartialEq, B> PartialEq for FiniteFloat<T, B> {
            fn eq(&self, other: &Self) -> bool {
                self.value == other.value
            }
        }

        impl<T: PartialEq, B> Eq for FiniteFloat<T, B> {}

        impl<T: PartialOrd, B> Ord for FiniteFloat<T, B> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.value
                    .partial_cmp(&other.value)
                    .expect("FiniteFloat values should always be comparable")
            }
        }

        impl<T: PartialOrd, B> PartialOrd for FiniteFloat<T, B> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // Formatting implementations
        impl<T: fmt::Display, B> fmt::Display for FiniteFloat<T, B> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl<T: fmt::Debug, B> fmt::Debug for FiniteFloat<T, B> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "FiniteFloat({:?})", self.value)
            }
        }
    }
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
                        type Output = Option<#output_alias>;

                        fn #method_ident(self, rhs: #rhs_alias) -> Self::Output {
                            let rhs_val = rhs.get();
                            if rhs_val == 0.0 {
                                return None;
                            }
                            let result = self.get() / rhs_val;
                            #output_alias::new(result)
                        }
                    }
                }
            } else {
                // Potentially failing operation: return Option
                quote! {
                    impl #trait_ident<#rhs_alias> for #lhs_alias {
                        type Output = Option<#output_alias>;

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
/// - `Lhs op Option<Rhs>` -> Option<Output>
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
        |lhs_alias,
         rhs_alias,
         output_alias,
         trait_ident,
         method_ident,
         _op_symbol,
         _result,
         _op| {
            // Lhs op Option<Rhs> -> Option<Output>
            // This is allowed because Lhs is a local type
            quote! {
                impl #trait_ident<Option<#rhs_alias>> for #lhs_alias {
                    type Output = Option<#output_alias>;

                    fn #method_ident(self, rhs: Option<#rhs_alias>) -> Self::Output {
                        match rhs {
                            Some(b) => {
                                let inner_result = self.#method_ident(b);
                                inner_result.into()
                            }
                            None => None,
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

/// Generates type aliases.
pub fn generate_type_aliases(config: &TypeConfig) -> TokenStream2 {
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
pub fn generate_new_const_methods(config: &TypeConfig) -> TokenStream2 {
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
