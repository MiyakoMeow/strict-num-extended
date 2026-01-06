//! `FiniteFloat` struct and basic methods module

use quote::quote;

/// Generates `FiniteFloat` struct and basic methods.
pub fn generate_finite_float_struct() -> proc_macro2::TokenStream {
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
            /// use strict_num_extended::FinF32;
            ///
            /// let finite = FinF32::new(3.14);
            /// assert_eq!(finite.unwrap().get(), 3.14);
            /// ```
            ///
            /// Returns `None` if the value does not satisfy the constraint.
            #[must_use]
            pub fn new(value: T) -> Option<Self> {
                let val_f64: f64 = value.into();

                let in_bounds = val_f64.is_finite()
                    && val_f64 >= Self::MIN
                    && val_f64 <= Self::MAX;

                let not_zero = if EXCLUDE_ZERO {
                    val_f64 != 0.0
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
                U: Copy + Into<f64>,
                T: From<U>,
            {
                Self::new(T::from(value)).ok_or(())
            }
        }
    }
}
