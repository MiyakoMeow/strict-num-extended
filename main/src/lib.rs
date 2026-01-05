#![allow(clippy::manual_range_contains)]

//! # Finite Floating-Point Types
//!
//! This module provides finite floating-point types. All types guarantee finite values
//! (excluding NaN and infinity) **automatically** - no need to manually specify `is_finite()`:
//! - `FinF32` and `FinF64`: Finite floating-point numbers (excludes NaN and infinity)
//! - `PositiveF32` and `PositiveF64`: Non-negative floating-point numbers (>= 0, finite)
//! - `NonZeroF32` and `NonZeroF64`: Non-zero floating-point numbers (!= 0, excludes 0.0, -0.0, NaN, infinity)
//! - `NonZeroPositiveF32` and `NonZeroPositiveF64`: Non-zero positive floating-point numbers (> 0, finite)
//! - `NegativeF32` and `NegativeF64`: Non-positive floating-point numbers (<= 0, finite)
//! - `NonZeroNegativeF32` and `NonZeroNegativeF64`: Non-zero negative floating-point numbers (< 0, finite)
//! - `NormalizedF32` and `NormalizedF64`: Normalized floating-point numbers (0.0 <= value <= 1.0, finite)
//! - `NegativeNormalizedF32` and `NegativeNormalizedF64`: Negative normalized floating-point numbers (-1.0 <= value <= 0.0, finite)
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
//! ## Quick Overview
//!
//! ```
//! use strict_num_extended::{FinF32, PositiveF32, NonZeroPositiveF32};
//!
//! // Create finite floating-point numbers (no NaN or infinity)
//! let finite = FinF32::new(3.14).unwrap();
//! assert_eq!(finite.get(), 3.14);
//!
//! // Rejected: NaN and infinity are not allowed
//! assert!(FinF32::new(f32::NAN).is_none());
//! assert!(FinF32::new(f32::INFINITY).is_none());
//!
//! // Positive numbers (>= 0)
//! let positive = PositiveF32::new(42.0).unwrap();
//! assert!(positive >= PositiveF32::new(0.0).unwrap());
//!
//! // Non-zero positive numbers (> 0)
//! let nonzero_pos = NonZeroPositiveF32::new(10.0).unwrap();
//! assert!(nonzero_pos.get() > 0.0);
//! ```
//!
//! ## Arithmetic Operations
//!
//! Arithmetic operations automatically validate results and preserve constraints:
//!
//! ```
//! use strict_num_extended::NonZeroPositiveF32;
//!
//! let a = NonZeroPositiveF32::new(10.0).unwrap();
//! let b = NonZeroPositiveF32::new(20.0).unwrap();
//!
//! // Addition preserves constraint (result must still be > 0)
//! let sum = a + b;
//! assert_eq!(sum.get(), 30.0);
//!
//! // Subtraction panics if result violates constraint
//! // let invalid = b - a; // This would panic if result <= 0
//!
//! // Multiplication preserves constraint
//! let product = a * b;
//! assert_eq!(product.get(), 200.0);
//! ```
//!
//! ## Comparison Operations
//!
//! All types support full ordering operations:
//!
//! ```
//! use strict_num_extended::PositiveF32;
//!
//! let a = PositiveF32::new(5.0).unwrap();
//! let b = PositiveF32::new(10.0).unwrap();
//!
//! assert!(a < b);
//! assert!(b > a);
//! assert!(a <= b);
//! assert!(b >= a);
//! assert_ne!(a, b);
//! ```
//!
//! # Option Types
//!
//! For handling potentially failing operations, `Option` versions of all types are provided:
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
//! ## Practical Example: Safe Division
//!
//! Option types are particularly useful for functions that may fail:
//!
//! ```
//! use strict_num_extended::{OptFinF32, OptNonZeroF32, FinF32, NonZeroF32};
//!
//! fn safe_divide(a: OptFinF32, b: OptNonZeroF32) -> OptFinF32 {
//!     match (a, b) {
//!         (Some(num), Some(denom)) => {
//!             // Convert to f32, divide, then wrap back in FinF32
//!             FinF32::new(num.get() / denom.get())
//!         },
//!         _ => None,
//!     }
//! }
//!
//! let result = safe_divide(
//!     Some(FinF32::new(10.0).unwrap()),
//!     Some(NonZeroF32::new(2.0).unwrap())
//! );
//! assert!(result.is_some());
//! assert_eq!(result.unwrap().get(), 5.0);
//!
//! // Division by zero is prevented - NonZeroF32 rejects 0.0
//! let zero_denom: OptNonZeroF32 = NonZeroF32::new(0.0);
//! assert!(zero_denom.is_none());
//!
//! let invalid = safe_divide(
//!     Some(FinF32::new(10.0).unwrap()),
//!     zero_denom
//! );
//! assert!(invalid.is_none());
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
//!
//! # How Constraints Work
//!
//! All types automatically reject special floating-point values that could cause bugs.
//! This validation happens at creation time, ensuring type safety throughout your program.
//!
//! ## Finite Constraint
//!
//! The `Fin` types exclude only NaN and infinity, accepting all other finite values:
//!
//! ```
//! use strict_num_extended::FinF32;
//!
//! let valid = FinF32::new(3.14);         // ✓ Some(value)
//! let invalid = FinF32::new(f32::NAN);    // ✗ None
//! let invalid = FinF32::new(f32::INFINITY); // ✗ None
//! ```
//!
//! ## Positive Constraint
//!
//! The `Positive` types require finite AND non-negative values (x ≥ 0):
//!
//! ```
//! use strict_num_extended::PositiveF32;
//!
//! let valid = PositiveF32::new(0.0);      // ✓ Some(value)
//! let valid = PositiveF32::new(1.5);      // ✓ Some(value)
//! let invalid = PositiveF32::new(-1.0);   // ✗ None (negative)
//! let invalid = PositiveF32::new(f32::INFINITY); // ✗ None (infinite)
//! ```
//!
//! ## Negative Constraint
//!
//! The `Negative` types require finite AND non-positive values (x ≤ 0):
//!
//! ```
//! use strict_num_extended::NegativeF32;
//!
//! let valid = NegativeF32::new(0.0);      // ✓ Some(value)
//! let valid = NegativeF32::new(-1.5);     // ✓ Some(value)
//! let invalid = NegativeF32::new(1.0);    // ✗ None (positive)
//! let invalid = NegativeF32::new(f32::NEG_INFINITY); // ✗ None (infinite)
//! ```
//!
//! ## `NonZero` Constraint
//!
//! The `NonZero` types require finite AND non-zero values (x ≠ 0), excluding both +0.0 and -0.0:
//!
//! ```
//! use strict_num_extended::NonZeroF32;
//!
//! let valid = NonZeroF32::new(1.0);       // ✓ Some(value)
//! let valid = NonZeroF32::new(-1.0);      // ✓ Some(value)
//! let invalid = NonZeroF32::new(0.0);     // ✗ None (zero)
//! let invalid = NonZeroF32::new(-0.0);    // ✗ None (negative zero)
//! ```
//!
//! ## Normalized Constraint
//!
//! The `Normalized` types require finite values in [0.0, 1.0]:
//!
//! ```
//! use strict_num_extended::NormalizedF32;
//!
//! let valid = NormalizedF32::new(0.75);   // ✓ Some(value)
//! let valid = NormalizedF32::new(0.0);    // ✓ Some(value)
//! let valid = NormalizedF32::new(1.0);    // ✓ Some(value)
//! let invalid = NormalizedF32::new(1.5);  // ✗ None (> 1.0)
//! let invalid = NormalizedF32::new(-0.5); // ✗ None (< 0.0)
//! ```
//!
//! ## `NegativeNormalized` Constraint
//!
//! The `NegativeNormalized` types require finite values in [-1.0, 0.0]:
//!
//! ```
//! use strict_num_extended::NegativeNormalizedF32;
//!
//! let valid = NegativeNormalizedF32::new(-0.75);  // ✓ Some(value)
//! let valid = NegativeNormalizedF32::new(-1.0);   // ✓ Some(value)
//! let valid = NegativeNormalizedF32::new(0.0);    // ✓ Some(value)
//! let invalid = NegativeNormalizedF32::new(1.5);  // ✗ None (> 0.0)
//! let invalid = NegativeNormalizedF32::new(-1.5); // ✗ None (< -1.0)
//! ```
//!
//! ## Combined Constraints
//!
//! Combined types enforce multiple constraints simultaneously:
//!
//! ```
//! use strict_num_extended::NonZeroPositiveF32;
//!
//! // NonZeroPositive = Positive AND NonZero (equivalent to x > 0)
//! let valid = NonZeroPositiveF32::new(1.0);     // ✓ Some(value)
//! let valid = NonZeroPositiveF32::new(0.001);   // ✓ Some(value)
//! let invalid = NonZeroPositiveF32::new(0.0);   // ✗ None (zero)
//! let invalid = NonZeroPositiveF32::new(-1.0);  // ✗ None (negative)
//! ```
//!
//! ## Unary Negation
//!
//! The unary negation operator (`-`) is supported with automatic type inference:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Positive ↔ Negative
//! let pos = PositiveF64::new(5.0).unwrap();
//! let neg: NegativeF64 = -pos;
//! assert_eq!(neg.get(), -5.0);
//!
//! // NonZeroPositive ↔ NonZeroNegative
//! let nz_pos = NonZeroPositiveF32::new(10.0).unwrap();
//! let nz_neg: NonZeroNegativeF32 = -nz_pos;
//! assert_eq!(nz_neg.get(), -10.0);
//!
//! // Normalized ↔ NegativeNormalized
//! let norm = NormalizedF64::new(0.75).unwrap();
//! let neg_norm: NegativeNormalizedF64 = -norm;
//! assert_eq!(neg_norm.get(), -0.75);
//!
//! // Fin is reflexive (negating Fin returns Fin)
//! let fin = FinF32::new(2.5).unwrap();
//! let neg_fin: FinF32 = -fin;
//! assert_eq!(neg_fin.get(), -2.5);
//! ```

// Generate all code using proc_macro
strict_num_extended_macros::generate_finite_float_types!([
    (Fin, []),
    (Positive, [">= 0.0"]),
    (Negative, ["<= 0.0"]),
    (NonZero, ["!= 0.0"]),
    (Normalized, [">= 0.0", "<= 1.0"]),
    (NegativeNormalized, [">= -1.0", "<= 0.0"]),
    (NonZeroPositive, ["> 0.0"]),
    (NonZeroNegative, ["< 0.0"]),
]);
