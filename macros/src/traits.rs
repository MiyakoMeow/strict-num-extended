//! `FloatBase` trait and constant definitions module

use quote::quote;

/// Generates `FloatBase` trait and constants.
pub fn generate_float_base_trait() -> proc_macro2::TokenStream {
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
