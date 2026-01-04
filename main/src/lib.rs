#![allow(clippy::manual_range_contains)]

//! # Constrained Floating-Point Types
//!
//! This module provides constrained floating-point types. All types guarantee finite values
//! (excluding NaN and infinity) by default:
//! - `FinF32` and `FinF64`: Finite floating-point numbers (excludes NaN and infinity)
//! - `PositiveF32` and `PositiveF64`: Non-negative floating-point numbers (>= 0, finite)
//! - `NonZeroF32` and `NonZeroF64`: Non-zero floating-point numbers (!= 0, excludes 0.0, -0.0, NaN, infinity)
//! - `NonZeroPositiveF32` and `NonZeroPositiveF64`: Non-zero positive floating-point numbers (> 0, finite)
//! - `NegativeF32` and `NegativeF64`: Non-positive floating-point numbers (<= 0, finite)
//! - `NonZeroNegativeF32` and `NonZeroNegativeF64`: Non-zero negative floating-point numbers (< 0, finite)
//! - `NormalizedF32` and `NormalizedF64`: Normalized floating-point numbers (0.0 <= value <= 1.0, finite)
//!
//! ## Composable Constraints
//!
//! All constraints can be freely combined. For example, `NonZeroPositiveF32` combines
//! `Positive` and `NonZero` constraints:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Use predefined combined types
//! let nonzero_pos: NonZeroPositiveF32 = NonZeroPositiveF32::new(10.0).unwrap();
//! ```
//!
//! Additionally, `Option` versions are provided for handling potentially failing operations.
//!
//! # Examples
//!
//! ```
//! use strict_num_extended::{
//!     FinF32, PositiveF32, NonZeroF32, NonZeroPositiveF32,
//!     NegativeF32, NonZeroNegativeF32, NormalizedF32
//! };
//!
//! let finite = FinF32::new(3.14).unwrap();
//! let positive = PositiveF32::new(42.0).unwrap();
//! let non_zero = NonZeroF32::new(5.0).unwrap();
//! let non_zero_positive = NonZeroPositiveF32::new(10.0).unwrap();
//! let negative = NegativeF32::new(-5.0).unwrap();
//! let non_zero_negative = NonZeroNegativeF32::new(-10.0).unwrap();
//! let normalized = NormalizedF32::new(0.75).unwrap();
//! assert_eq!(finite.get(), 3.14);
//! assert_eq!(positive.get(), 42.0);
//! assert_eq!(non_zero.get(), 5.0);
//! assert_eq!(non_zero_positive.get(), 10.0);
//! assert_eq!(negative.get(), -5.0);
//! assert_eq!(non_zero_negative.get(), -10.0);
//! assert_eq!(normalized.get(), 0.75);
//! ```
//!
//! # Option Types
//!
//! ```
//! use strict_num_extended::{FinF32, OptFinF32, OptPositiveF32};
//!
//! let a: OptFinF32 = Some(FinF32::new(1.0).unwrap());
//! let b: OptFinF32 = None;
//! assert!(a.is_some());
//! assert!(b.is_none());
//! ```
//!
//! # Compile-Time Constants
//!
//! ```
//! use strict_num_extended::FinF32;
//!
//! const ONE: FinF32 = FinF32::new_const(1.0);
//! assert_eq!(ONE.get(), 1.0);
//! ```
//!
//! **Note**: The `new_const` method now supports compile-time validation and will panic at
//! compile time if the value does not satisfy the constraint conditions.

// Generate all code using proc_macro
strict_num_extended_macros::generate_constrained_types!({
    // Atomic constraint types
    constraints: [
        Finite {
            doc: "Finite floating-point value (excludes NaN and infinity)",
            validate: "!value.is_nan() && !value.is_infinite()"
        },
        Positive {
            doc: "Non-negative floating-point value (>= 0, finite)",
            validate: "!value.is_nan() && !value.is_infinite() && !value.is_sign_negative()"
        },
        Negative {
            doc: "Non-positive floating-point value (<= 0, finite)",
            validate: "!(value.is_nan() || value.is_infinite() || (value.is_sign_positive() && value != 0.0))"
        },
        NonZero {
            doc: "Non-zero floating-point value (!= 0.0 && != -0.0)",
            validate: "!value.is_nan() && !value.is_infinite() && value != 0.0 && value != -0.0"
        },
        Normalized {
            doc: "Normalized floating-point value (0.0 <= value <= 1.0, finite)",
            validate: "!value.is_nan() && !value.is_infinite() && value >= 0.0 && value <= 1.0"
        }
    ],

    // Type definitions (uniformly use square brackets)
    constraint_types: [
        // Single constraint types
        (Fin, [f32, f64], [Finite]),
        (Positive, [f32, f64], [Positive]),
        (Negative, [f32, f64], [Negative]),
        (NonZero, [f32, f64], [NonZero]),
        (Normalized, [f32, f64], [Normalized]),
        // Combined constraint types
        (NonZeroPositive, [f32, f64], [Positive, NonZero]),
        (NonZeroNegative, [f32, f64], [Negative, NonZero]),
    ]
});
