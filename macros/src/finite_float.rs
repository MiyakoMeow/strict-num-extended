//! `FiniteFloat` struct and basic methods module

use quote::quote;

/// Generates `FiniteFloat` struct and basic methods.
pub fn generate_finite_float_struct() -> proc_macro2::TokenStream {
    quote! {
        /// Generic finite floating-point structure
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct FiniteFloat<T, B> {
            value: T,
            _marker: PhantomData<B>,
        }

        impl<T, const MIN_BITS: i64, const MAX_BITS: i64, const EXCLUDE_ZERO: bool>
            FiniteFloat<T, Bounded<MIN_BITS, MAX_BITS, EXCLUDE_ZERO>>
        where
            T: Copy + Into<f64>,
        {
            /// Decodes boundary constants from bit representation
            const MIN: f64 = f64::from_bits(MIN_BITS as u64);
            const MAX: f64 = f64::from_bits(MAX_BITS as u64);

            /// Creates a new finite floating-point number
            ///
            /// # Example
            ///
            /// ```
            /// use strict_num_extended::{FinF32, FloatError};
            ///
            /// let finite = FinF32::new(3.14)?;
            /// assert_eq!(finite.get(), 3.14);
            /// # Ok::<(), FloatError>(())
            /// ```
            ///
            /// # Errors
            ///
            /// Returns `Err(FloatError)` if the value does not satisfy the constraint.
            #[must_use]
            pub fn new(value: T) -> Result<Self, FloatError> {
                let val_f64: f64 = value.into();

                // Check for NaN
                if val_f64.is_nan() {
                    return Err(FloatError::NaN);
                }

                // Check for positive infinity
                if val_f64.is_infinite() && val_f64 > 0.0 {
                    return Err(FloatError::PosInf);
                }

                // Check for negative infinity
                if val_f64.is_infinite() && val_f64 < 0.0 {
                    return Err(FloatError::NegInf);
                }

                let in_bounds = val_f64 >= Self::MIN && val_f64 <= Self::MAX;
                let not_zero = if EXCLUDE_ZERO {
                    val_f64 != 0.0
                } else {
                    true
                };

                if in_bounds && not_zero {
                    Ok(Self {
                        value,
                        _marker: PhantomData,
                    })
                } else {
                    Err(FloatError::OutOfRange)
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
        }

        #[cfg(feature = "serde")]
        impl<'de, T, const MIN_BITS: i64, const MAX_BITS: i64, const EXCLUDE_ZERO: bool>
            serde::Deserialize<'de> for FiniteFloat<T, Bounded<MIN_BITS, MAX_BITS, EXCLUDE_ZERO>>
        where
            T: Copy + Into<f64> + serde::Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                // First deserialize the raw value
                let value = T::deserialize(deserializer)?;

                // Then validate using the new() method
                Self::new(value).map_err(|e| {
                    use serde::de::Error;
                    match e {
                        FloatError::NaN => D::Error::custom("value is NaN"),
                        FloatError::PosInf => D::Error::custom("value is positive infinity"),
                        FloatError::NegInf => D::Error::custom("value is negative infinity"),
                        FloatError::OutOfRange => D::Error::custom("value is out of range"),
                        FloatError::NoneOperand => D::Error::custom("none operand"),
                    }
                })
            }
        }

        #[cfg(feature = "serde")]
        impl<T, const MIN_BITS: i64, const MAX_BITS: i64, const EXCLUDE_ZERO: bool> serde::Serialize
            for FiniteFloat<T, Bounded<MIN_BITS, MAX_BITS, EXCLUDE_ZERO>>
        where
            T: serde::Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.value.serialize(serializer)
            }
        }
    }
}
