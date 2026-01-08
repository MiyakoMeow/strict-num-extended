//! Constraint marker types and traits
//!
//! This module defines zero-sized marker types for float constraints.

use quote::quote;

/// Generates zero-sized constraint marker types
pub fn generate_constraint_markers() -> proc_macro2::TokenStream {
    quote! {
        /// Constraint marker: finite floating-point number (excludes NaN and infinity)
        #[derive(Debug, Clone, Copy)]
        pub struct Fin;

        /// Constraint marker: non-negative number (>= 0, finite)
        #[derive(Debug, Clone, Copy)]
        pub struct Positive;

        /// Constraint marker: non-positive number (<= 0, finite)
        #[derive(Debug, Clone, Copy)]
        pub struct Negative;

        /// Constraint marker: non-zero number (!= 0, excludes +0.0 and -0.0)
        #[derive(Debug, Clone, Copy)]
        pub struct NonZero;

        /// Constraint marker: normalized number ([0.0, 1.0], finite)
        #[derive(Debug, Clone, Copy)]
        pub struct Normalized;

        /// Constraint marker: negative normalized number ([-1.0, 0.0], finite)
        #[derive(Debug, Clone, Copy)]
        pub struct NegativeNormalized;

        /// Constraint marker: non-zero positive number (> 0, finite)
        #[derive(Debug, Clone, Copy)]
        pub struct NonZeroPositive;

        /// Constraint marker: non-zero negative number (< 0, finite)
        #[derive(Debug, Clone, Copy)]
        pub struct NonZeroNegative;

        /// Constraint marker: symmetric number ([-1.0, 1.0], finite)
        #[derive(Debug, Clone, Copy)]
        pub struct Symmetric;
    }
}
