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
//! - `SymmetricF32` and `SymmetricF64`: Symmetric floating-point numbers (-1.0 <= value <= 1.0, finite)
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
//! assert!(FinF32::new(f32::NAN).is_err());
//! assert!(FinF32::new(f32::INFINITY).is_err());
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
//! Arithmetic operations automatically validate results and return `Option` for potentially
//! failing operations. Safe operations (like bounded type multiplication) return direct values:
//!
//! ```
//! use strict_num_extended::NonZeroPositiveF32;
//!
//! let a = NonZeroPositiveF32::new(10.0).unwrap();
//! let b = NonZeroPositiveF32::new(20.0).unwrap();
//!
//! // Addition returns Option (overflow possible)
//! let sum = (a + b).unwrap();
//! assert_eq!(sum.get(), 30.0);
//!
//! // Multiplication returns Option (overflow possible for unbounded types)
//! let product = (a * b).unwrap();
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
//! # Error Handling
//!
//! All floating-point conversions return `Result<T, FloatError>` for proper error handling:
//!
//! ```
//! use strict_num_extended::{FinF32, NonZeroF32};
//!
//! let a: Result<FinF32, _> = FinF32::new(1.0);
//! let b: Result<NonZeroF32, _> = NonZeroF32::new(0.0);
//! assert!(a.is_ok());
//! assert!(b.is_err());
//! ```
//!
//! ## Practical Example: Safe Division
//!
//! Result types are particularly useful for functions that may fail:
//!
//! ```
//! use strict_num_extended::{FinF32, NonZeroF32, FloatError};
//!
//! fn safe_divide(a: Result<FinF32, FloatError>, b: Result<NonZeroF32, FloatError>) -> Result<FinF32, FloatError> {
//!     match (a, b) {
//!         (Ok(num), Ok(denom)) => {
//!             // Get the values, divide, then wrap back in FinF32
//!             FinF32::new(num.get() / denom.get())
//!         },
//!         (Err(e), _) => Err(e),
//!         (Ok(_), Err(e)) => Err(e),
//!     }
//! }
//!
//! let result = safe_divide(
//!     FinF32::new(10.0),
//!     NonZeroF32::new(2.0)
//! );
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap().get(), 5.0);
//!
//! // Division by zero is prevented - NonZeroF32 returns error for 0.0
//! let zero_denom: Result<NonZeroF32, FloatError> = NonZeroF32::new(0.0);
//! assert!(zero_denom.is_err());
//!
//! let invalid = safe_divide(
//!     FinF32::new(10.0),
//!     zero_denom
//! );
//! assert!(invalid.is_err());
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
//! ## `Symmetric` Constraint
//!
//! The `Symmetric` types require finite values in [-1.0, 1.0]:
//!
//! ```
//! use strict_num_extended::SymmetricF32;
//!
//! let valid = SymmetricF32::new(0.75);   // ✓ Some(value)
//! let valid = SymmetricF32::new(-0.5);   // ✓ Some(value)
//! let valid = SymmetricF32::new(1.0);    // ✓ Some(value)
//! let valid = SymmetricF32::new(-1.0);   // ✓ Some(value)
//! let invalid = SymmetricF32::new(1.5);  // ✗ None (> 1.0)
//! let invalid = SymmetricF32::new(-1.5); // ✗ None (< -1.0)
//! ```
//!
//! Note: The `Symmetric` type is reflexive under negation (negating a `Symmetric` returns `Symmetric`):
//!
//! ```
//! use strict_num_extended::SymmetricF64;
//!
//! let val = SymmetricF64::new(0.75).unwrap();
//! let neg_val: SymmetricF64 = -val;  // Still SymmetricF64
//! assert_eq!(neg_val.get(), -0.75);
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
    (Symmetric, [">= -1.0", "<= 1.0"]),
]);
