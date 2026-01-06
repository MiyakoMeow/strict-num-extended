//! Comparison and formatting trait implementations module

use quote::quote;

/// Generates comparison and formatting trait implementations.
pub fn generate_comparison_traits() -> proc_macro2::TokenStream {
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
