//! Unary operations module
//!
//! Generates type-safe unary operations for all constraint types, including:
//! - `abs()`: Absolute value operation with automatic output type inference
//! - `signum()`: Sign function that always returns Symmetric type

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};

use crate::config::{ConstraintDef, TypeConfig};

/// Generates type alias identifier for type and floating-point type
fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Finds constraint definition by type name
fn find_constraint_def<'a>(config: &'a TypeConfig, type_name: &Ident) -> &'a ConstraintDef {
    config
        .constraints
        .iter()
        .find(|c| &c.name == type_name)
        .expect("Constraint not found")
}

/// Checks if bounds represent Symmetric type [-1, 1]
fn is_symmetric_bounds(constraint_def: &ConstraintDef) -> bool {
    constraint_def.bounds.lower == Some(-1.0) && constraint_def.bounds.upper == Some(1.0)
}

/// Checks if bounds represent Normalized type [0, 1]
fn is_normalized_bounds(constraint_def: &ConstraintDef) -> bool {
    constraint_def.bounds.lower == Some(0.0) && constraint_def.bounds.upper == Some(1.0)
}

/// Checks if bounds represent `NegativeNormalized` type [-1, 0]
fn is_negative_normalized_bounds(constraint_def: &ConstraintDef) -> bool {
    constraint_def.bounds.lower == Some(-1.0) && constraint_def.bounds.upper == Some(0.0)
}

/// Infers the output type for `abs()` operation based on constraint properties
fn infer_abs_output_type(constraint_def: &ConstraintDef) -> Ident {
    let is_bounded = constraint_def.bounds.is_bounded();
    let excludes_zero = constraint_def.excludes_zero;

    // Special bounded cases first
    if is_bounded && !excludes_zero {
        if is_symmetric_bounds(constraint_def) || is_normalized_bounds(constraint_def) {
            // Symmetric [-1, 1] → Normalized [0, 1]
            // Normalized [0, 1] → Normalized [0, 1] (reflexive)
            return Ident::new("Normalized", Span::call_site());
        }
        if is_negative_normalized_bounds(constraint_def) {
            // NegativeNormalized [-1, 0] → Normalized [0, 1]
            return Ident::new("Normalized", Span::call_site());
        }
    }

    // General case: determine output type based on zero exclusion
    let output_type = if excludes_zero {
        "NonZeroPositive"
    } else {
        "Positive"
    };

    Ident::new(output_type, Span::call_site())
}

/// Infers the output type for `signum()` operation based on constraint properties
///
/// Rules:
/// - Positive types (>= 0) → signum in {0, 1} → Normalized
/// - Negative types (<= 0) → signum in {-1, 0} → `NegativeNormalized`
/// - Positive + excludes zero → signum = 1 → Normalized
/// - Negative + excludes zero → signum = -1 → `NegativeNormalized`
/// - Any + excludes zero (`NonZero`) → signum in {-1, 1} → Symmetric
/// - Any + includes zero (Fin, Symmetric) → signum in {-1, 0, 1} → Symmetric
fn infer_signum_output_type(constraint_def: &ConstraintDef) -> Ident {
    use crate::config::Sign;

    match (constraint_def.sign, constraint_def.excludes_zero) {
        // Positive types: signum ∈ {0, 1} or {1}
        (Sign::Positive, _) => Ident::new("Normalized", Span::call_site()),

        // Negative types: signum ∈ {-1, 0} or {-1}
        (Sign::Negative, _) => Ident::new("NegativeNormalized", Span::call_site()),

        // Any sign types
        (Sign::Any, true) => {
            // NonZero: signum ∈ {-1, 1} (no zero)
            // This fits in Symmetric [-1, 1]
            Ident::new("Symmetric", Span::call_site())
        }
        (Sign::Any, false) => {
            // Fin or Symmetric: signum ∈ {-1, 0, 1}
            // This requires Symmetric [-1, 1]
            Ident::new("Symmetric", Span::call_site())
        }
    }
}

/// Generates `abs()` method implementations for all constraint types
///
/// For each type, generates:
/// ```rust
/// impl #type_alias {
///     #[inline]
///     #[must_use]
///     pub fn abs(self) -> #output_alias {
///         let result = self.get().abs();
///         // SAFETY: ...
///         unsafe { #output_alias::new_unchecked(result) }
///     }
/// }
/// ```
pub fn generate_abs_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;
        let constraint_def = find_constraint_def(config, type_name);

        // Infer abs() output type
        let output_type = infer_abs_output_type(constraint_def);

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(quote! {
                impl #type_alias {
                    /// Computes the absolute value.
                    ///
                    /// The return type is automatically inferred based on the source constraint:
                    /// - `Positive`/`Negative` → `Positive`
                    /// - `NonZero` → `NonZeroPositive`
                    /// - `Normalized` → `Normalized` (reflexive)
                    /// - `Symmetric` → `Normalized`
                    /// - `Fin` → `Positive`
                    ///
                    /// # Examples
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let neg = NegativeF64::new(-5.0).unwrap();
                    /// let abs_val: PositiveF64 = neg.abs();
                    /// assert_eq!(abs_val.get(), 5.0);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn abs(self) -> #output_alias {
                        let result = self.get().abs();
                        // SAFETY: The output type for abs() was determined at compile time
                        // through constraint analysis. For any input value, its absolute value
                        // is guaranteed to satisfy the output type's constraints. Runtime
                        // validation would be redundant, so we use unchecked for performance.
                        unsafe { #output_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
}

/// Generates `signum()` method implementations for all constraint types
///
/// For each type, generates:
/// ```rust
/// impl #type_alias {
///     #[inline]
///     #[must_use]
///     pub fn signum(self) -> #output_alias {
///         let result = self.get().signum();
///         // SAFETY: ...
///         unsafe { #output_alias::new_unchecked(result) }
///     }
/// }
/// ```
pub fn generate_signum_impls(config: &TypeConfig) -> TokenStream2 {
    let mut impls = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;
        let constraint_def = find_constraint_def(config, type_name);

        // Infer signum() output type based on constraint properties
        let output_type = infer_signum_output_type(constraint_def);

        for float_type in &type_def.float_types {
            let type_alias = make_type_alias(type_name, float_type);
            let output_alias = make_type_alias(&output_type, float_type);

            impls.push(quote! {
                impl #type_alias {
                    /// Computes the sign function.
                    ///
                    /// Returns the sign of the number:
                    /// - `1.0` if the number is positive
                    /// - `0.0` if the number is zero
                    /// - `-1.0` if the number is negative
                    ///
                    /// The return type is automatically inferred based on the source constraint:
                    /// - `Positive` types → `Normalized` (signum in {0, 1})
                    /// - `Negative` types → `NegativeNormalized` (signum in {-1, 0})
                    /// - `NonZero` types → `Symmetric` (signum in {-1, 1})
                    /// - `Fin`/`Symmetric` → `Symmetric` (signum in {-1, 0, 1})
                    ///
                    /// # Examples
                    /// ```
                    /// use strict_num_extended::*;
                    ///
                    /// let pos = PositiveF64::new(5.0).unwrap();
                    /// let sign: NormalizedF64 = pos.signum();
                    /// assert_eq!(sign.get(), 1.0);
                    ///
                    /// let neg = NegativeF64::new(-5.0).unwrap();
                    /// let sign: NegativeNormalizedF64 = neg.signum();
                    /// assert_eq!(sign.get(), -1.0);
                    /// ```
                    #[inline]
                    #[must_use]
                    pub fn signum(self) -> #output_alias {
                        let result = self.get().signum();
                        // SAFETY: The output type for signum() was determined at compile time
                        // through constraint analysis. For any input value, its signum is
                        // guaranteed to satisfy the output type's constraints. Runtime
                        // validation would be redundant, so we use unchecked for performance.
                        unsafe { #output_alias::new_unchecked(result) }
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
}
