//! # Finite Floating-Point Types
//!
//! This module provides finite floating-point types. All types guarantee finite values
//! (excluding NaN and infinity) **automatically** - no need to manually specify `is_finite()`:
//! - `FinF32` and `FinF64`: Finite floating-point numbers (excludes NaN and infinity)
//! - `NonNegativeF32` and `NonNegativeF64`: Non-negative floating-point numbers (>= 0, finite)
//! - `NonZeroF32` and `NonZeroF64`: Non-zero floating-point numbers (!= 0, excludes 0.0, -0.0, NaN, infinity)
//! - `PositiveF32` and `PositiveF64`: Positive floating-point numbers (> 0, finite)
//! - `NonPositiveF32` and `NonPositiveF64`: Non-positive floating-point numbers (<= 0, finite)
//! - `NegativeF32` and `NegativeF64`: Negative floating-point numbers (< 0, finite)
//! - `NormalizedF32` and `NormalizedF64`: Normalized floating-point numbers (0.0 <= value <= 1.0, finite)
//! - `NegativeNormalizedF32` and `NegativeNormalizedF64`: Negative normalized floating-point numbers (-1.0 <= value <= 0.0, finite)
//! - `SymmetricF32` and `SymmetricF64`: Symmetric floating-point numbers (-1.0 <= value <= 1.0, finite)
//!
//! ## Feature Flags
//!
//! ### `serde` (optional)
//!
//! When the `serde` feature is enabled, all types implement `serde::Serialize` and
//! `serde::Deserialize` traits:
//!
//! Example usage with serde:
//!
//! ```rust,ignore
//! use strict_num_extended::FinF32;
//! use serde_json;
//!
//! let value = FinF32::new(3.14).unwrap();
//! let json = serde_json::to_string(&value).unwrap();
//! let deserialized: FinF32 = serde_json::from_str(&json).unwrap();
//! ```
//!
//! **Note**: This example requires the `std` feature (for `serde_json`).
//! For `no_std` environments with `serde`, use alternative serialization formats.
//!
//! ## Type Safety
//!
//! This library provides type safety through both compile-time and runtime guarantees:
//!
//! ### Compile-Time Safety
//!
//! Create constants at compile time with guaranteed validity:
//!
//! ```
//! use strict_num_extended::*;
//!
//! const MAX_VALUE: PositiveF64 = PositiveF64::new_const(100.0);
//! const HALF: NormalizedF32 = NormalizedF32::new_const(0.5);
//! ```
//!
//! The `new_const()` method validates constraints at compile time, ensuring invalid values
//! are caught before your code even runs.
//!
//! ### Runtime Safety
//!
//! At runtime, all operations automatically:
//! - **Validate value ranges** when creating instances
//! - **Detect overflow** in arithmetic operations
//! - **Return detailed errors** via `Result<T, FloatError>` for any violation
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Value creation validates ranges
//! let value = PositiveF64::new(42.0);
//! assert!(value.is_ok());
//!
//! // Invalid value returns error
//! let invalid = PositiveF64::new(-1.0);
//! assert!(invalid.is_err());
//! ```
//!
//! This two-layer safety approach ensures your floating-point code is correct both at
//! compile time and at runtime, catching errors early and preventing undefined behavior.
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Arithmetic detects overflow
//! let a = PositiveF64::new(1e308).unwrap();
//! let b = PositiveF64::new(1e308).unwrap();
//! let result = a + b;  // Returns Result, detects overflow
//! assert!(result.is_err());
//! ```
//!
//! ### Basic Usage
//!
//! ```
//! use strict_num_extended::{FinF32, NonNegativeF32, PositiveF32, SymmetricF32};
//!
//! // Create finite floating-point numbers (no NaN or infinity)
//! const FINITE: FinF32 = FinF32::new_const(3.14);
//! assert_eq!(FINITE.get(), 3.14);
//!
//! // Rejected: NaN and infinity are not allowed
//! assert!(FinF32::new(f32::NAN).is_err());
//! assert!(FinF32::new(f32::INFINITY).is_err());
//!
//! // Non-negative numbers (>= 0)
//! const NON_NEGATIVE: NonNegativeF32 = NonNegativeF32::new_const(42.0);
//! assert!(NON_NEGATIVE >= NonNegativeF32::new_const(0.0));
//!
//! // Positive numbers (> 0)
//! const POSITIVE: PositiveF32 = PositiveF32::new_const(10.0);
//! assert!(POSITIVE.get() > 0.0);
//!
//! // Arithmetic operations preserve constraints
//! let result = (POSITIVE + POSITIVE).unwrap();
//! assert_eq!(result.get(), 20.0);
//!
//! // Symmetric numbers in range [-1.0, 1.0]
//! const SYMMETRIC: SymmetricF32 = SymmetricF32::new_const(0.75);
//! assert_eq!(SYMMETRIC.get(), 0.75);
//!
//! // Negation is reflexive (Symmetric → Symmetric)
//! let negated: SymmetricF32 = -SYMMETRIC;
//! assert_eq!(negated.get(), -0.75);
//! ```
//!
//! ## Composable Constraints
//!
//! All constraints can be freely combined. For example, `PositiveF32` combines
//! `Positive` (> 0) constraint:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Use predefined combined types
//! let positive: PositiveF32 = PositiveF32::new(10.0).unwrap();
//! ```
//!
//! Additionally, `Option` versions are provided for handling potentially failing operations.
//!
//! # Examples
//!
//! ## Quick Overview
//!
//! ```
//! use strict_num_extended::{FinF32, NonNegativeF32, PositiveF32};
//!
//! // Create finite floating-point numbers (no NaN or infinity)
//! const FINITE: FinF32 = FinF32::new_const(3.14);
//! assert_eq!(FINITE.get(), 3.14);
//!
//! // Rejected: NaN and infinity are not allowed
//! assert!(FinF32::new(f32::NAN).is_err());
//! assert!(FinF32::new(f32::INFINITY).is_err());
//!
//! // Non-negative numbers (>= 0)
//! const NON_NEGATIVE: NonNegativeF32 = NonNegativeF32::new_const(42.0);
//! assert!(NON_NEGATIVE >= NonNegativeF32::new_const(0.0));
//!
//! // Positive numbers (> 0)
//! const POSITIVE: PositiveF32 = PositiveF32::new_const(10.0);
//! assert!(POSITIVE.get() > 0.0);
//! ```
//!
//! ## Type Conversions
//!
//! ### From/TryFrom Traits
//!
//! Conversions between constraint types and primitive types are provided through
//! the standard `From` and `TryFrom` traits:
//!
//! ```
//! use strict_num_extended::*;
//! use std::convert::TryInto;
//!
//! // 1. Constraint type → Primitive (always succeeds)
//! let fin = FinF32::new(2.5).unwrap();
//! let f32_val: f32 = fin.into();  // Using From trait
//! assert_eq!(f32_val, 2.5);
//!
//! // 2. Primitive → Constraint type (validated)
//! let fin: Result<FinF32, _> = FinF32::try_from(3.14);
//! assert!(fin.is_ok());
//!
//! let invalid: Result<FinF32, _> = FinF32::try_from(f32::NAN);
//! assert!(invalid.is_err());
//! ```
//!
//! ### Subset → Superset Conversions
//!
//! Conversions from more constrained types to less constrained types use `From`
//! and always succeed:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Normalized → Fin (subset to superset)
//! let normalized = NormalizedF32::new(0.5).unwrap();
//! let fin: FinF32 = normalized.into();  // Always succeeds
//! assert_eq!(fin.get(), 0.5);
//!
//! // NonNegative → Fin
//! let non_negative = NonNegativeF64::new(42.0).unwrap();
//! let fin: FinF64 = non_negative.into();
//! ```
//!
//! ### Superset → Subset Conversions
//!
//! Conversions from less constrained types to more constrained types use `TryFrom`
//! and may fail:
//!
//! ```
//! use strict_num_extended::*;
//! use std::convert::TryInto;
//!
//! // Fin → Normalized (may fail)
//! let fin = FinF64::new(0.5).unwrap();
//! let normalized: Result<NormalizedF64, _> = fin.try_into();
//! assert!(normalized.is_ok());
//!
//! let fin_out_of_range = FinF64::new(2.0).unwrap();
//! let normalized: Result<NormalizedF64, _> = fin_out_of_range.try_into();
//! assert!(normalized.is_err());  // Out of range
//! ```
//!
//! ### F32 ↔ F64 Conversions
//!
//! Conversions between F32 and F64 types are supported with precision awareness:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // F32 → F64 (always safe, lossless)
//! let fin32 = FinF32::new(3.0).unwrap();
//! let fin64: FinF64 = fin32.into();  // From trait
//! assert_eq!(fin64.get(), 3.0);
//!
//! // F64 → F32 (may overflow or lose precision)
//! let fin64_small = FinF64::new(3.0).unwrap();
//! let fin32: Result<FinF32, _> = fin64_small.try_into();
//! assert!(fin32.is_ok());
//!
//! let fin64_large = FinF64::new(1e40).unwrap();
//! let fin32: Result<FinF32, _> = fin64_large.try_into();
//! assert!(fin32.is_err());  // F32 range overflow
//! ```
//!
//! ## F32/F64 Conversion Methods
//!
//! For explicit conversions between F32 and F64 types, specialized methods are provided:
//!
//! ### `try_into_f32_type()` - F64 → F32 with Constraint Validation
//!
//! The `try_into_f32_type()` method converts F64 types to F32 with constraint validation:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Success: value fits in F32 and satisfies constraint
//! let f64_val = FinF64::new(3.0).unwrap();
//! let f32_val: Result<FinF32, _> = f64_val.try_into_f32_type();
//! assert!(f32_val.is_ok());
//!
//! // Success: precision loss is allowed
//! let f64_precise = FinF64::new(1.234_567_890_123_456_7).unwrap();
//! let f32_result: Result<FinF32, _> = f64_precise.try_into_f32_type();
//! assert!(f32_result.is_ok());  // Truncated to f32, but still valid
//!
//! // Error: F32 range overflow (becomes infinity)
//! let f64_large = FinF64::new(1e40).unwrap();
//! let f32_result: Result<FinF32, _> = f64_large.try_into_f32_type();
//! assert!(f32_result.is_err());  // Infinity is rejected
//! ```
//!
//! ### `as_f64_type()` - F32 → F64 Lossless Conversion
//!
//! The `as_f64_type()` method converts F32 types to F64 without loss of precision:
//!
//! ```
//! use strict_num_extended::*;
//!
//! let f32_val = FinF32::new(2.5).unwrap();
//! let f64_val: FinF64 = f32_val.as_f64_type();  // Always succeeds
//! assert_eq!(f64_val.get(), 2.5);
//!
//! // Roundtrip: F32 → F64 → F32
//! let original_f32 = FinF32::new(2.5).unwrap();
//! let f64_val: FinF64 = original_f32.as_f64_type();
//! let back_to_f32: Result<FinF32, _> = f64_val.try_into_f32_type();
//! assert!(back_to_f32.is_ok());
//! assert_eq!(back_to_f32.unwrap().get(), original_f32.get());
//! ```
//!
//! These methods support const contexts for creating values:
//!
//! ```
//! use strict_num_extended::*;
//!
//! const F64_VAL: FinF64 = FinF64::new_const(42.0);
//! const F32_VAL: FinF32 = FinF32::new_const(3.0);
//! ```
//!
//! > **Type Inference for Arithmetic Operations**: The library automatically infers the appropriate
//! > result type for arithmetic operations based on operand properties. The library supports
//! > standard Rust `From` and `TryFrom` traits for seamless conversions between constraint types,
//! > along with F32 ↔ F64 conversions with precision awareness. See the detailed examples above
//! > for conversion rules and patterns.
//!
//! ## Arithmetic Operations
//!
//! Arithmetic operations automatically validate results and return either the target type
//! or `Result<T, FloatError>` for potentially failing operations. Safe operations (like
//! bounded type multiplication) return direct values:
//!
//! ```
//! use strict_num_extended::PositiveF32;
//!
//! const A: PositiveF32 = PositiveF32::new_const(10.0);
//! const B: PositiveF32 = PositiveF32::new_const(20.0);
//!
//! // Addition returns Result (overflow possible)
//! let sum = (A + B).unwrap();
//! assert_eq!(sum.get(), 30.0);
//!
//! // Multiplication returns Result (overflow possible for unbounded types)
//! let product = (A * B).unwrap();
//! assert_eq!(product.get(), 200.0);
//! ```
//!
//! ### Type Inference and Conversion Rules
//!
//! Arithmetic operations automatically infer the appropriate result type based on the operands'
//! properties. The type system guarantees correctness through compile-time and runtime validation.
//!
//! #### Operation Safety Classification
//!
//! Operations are classified as either **safe** or **fallible**:
//!
//! - **Safe Operations**: Always produce valid results within the inferred type's constraints.
//!   Return the result directly without `Result` wrapping.
//! - **Fallible Operations**: May produce invalid results (overflow, division by zero, etc.).
//!   Return `Result<T, FloatError>` for explicit error handling.
//!
//! #### Type Inference Rules
//!
//! The result type is determined by analyzing:
//!
//! 1. **Sign Properties** (Positive, Negative, Any)
//! 2. **Bound Properties** (bounded/unbounded ranges)
//! 3. **Zero Exclusion** (whether the type excludes zero)
//!
//! **Addition Rules**:
//!
//! - `Positive + Positive → Positive` (fallible, may overflow)
//! - `Negative + Negative → Negative` (fallible, may overflow)
//! - `Positive + Negative → Fin` (safe, result has no sign constraint)
//! - `NonZero + NonZero (same sign) → NonZero` (fallible, may overflow)
//! - `NonZero + NonZero (different signs) → Fin` (safe)
//!
//! **Subtraction Rules**:
//!
//! - `Positive - Positive → Fin` (safe, result may have different sign)
//! - `Negative - Negative → Fin` (safe, result may have different sign)
//! - `Positive - Negative → Positive` (fallible, may overflow)
//! - `Negative - Positive → Negative` (fallible, may overflow)
//! - `NonZero - NonZero → Fin` (fallible if same sign, safe if different signs)
//!
//! **Multiplication Rules**:
//!
//! - `Positive × Positive → Positive` (fallible)
//! - `Negative × Negative → Positive` (fallible)
//! - `Positive × Negative → Negative` (fallible)
//! - `Bounded × Bounded → Bounded` (safe, result bounds computed from operand bounds)
//! - `NonZero × NonZero → NonZero` (fallible)
//!
//! **Division Rules**:
//!
//! - `Positive ÷ Positive → Positive` (fallible, division by zero detection)
//! - `Negative ÷ Negative → Positive` (fallible, division by zero detection)
//! - `Positive ÷ Negative → Negative` (fallible, division by zero detection)
//! - `Bounded (in [-1,1]) ÷ NonZero → Bounded` (safe, when dividend is within [-1,1])
//! - `NonZero ÷ NonZero → NonZero` (fallible, division by zero detection)
//!
//! #### Examples
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Safe operation: returns direct value
//! let a = PositiveF64::new(5.0).unwrap();
//! let b = NegativeF64::new(-3.0).unwrap();
//! let result: FinF64 = a + b;  // Returns FinF64 directly
//! assert_eq!(result.get(), 2.0);
//!
//! // Fallible operation: returns Result
//! let x = PositiveF64::new(10.0).unwrap();
//! let y = PositiveF64::new(5.0).unwrap();
//! let result: Result<PositiveF64, FloatError> = x + y;  // May overflow
//! assert!(result.is_ok());
//!
//! // Safe bounded multiplication
//! let norm1 = NormalizedF64::new(0.5).unwrap();
//! let norm2 = NormalizedF64::new(0.4).unwrap();
//! let product: NormalizedF64 = norm1 * norm2;  // Always in [0,1]
//! assert_eq!(product.get(), 0.2);
//!
//! // Error propagation in Result operations
//! let valid: Result<PositiveF64, FloatError> = Ok(PositiveF64::new(10.0).unwrap());
//! const B: NegativeF64 = NegativeF64::new_const(-3.0);
//! let result: Result<FinF64, FloatError> = valid + B;  // Result + Concrete
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap().get(), 7.0);
//!
//! // Error propagation when Result is Err
//! let invalid: Result<PositiveF64, FloatError> = Err(FloatError::OutOfRange);
//! let error_result: Result<FinF64, FloatError> = invalid + B;  // Error propagates
//! assert!(error_result.is_err());
//! ```
//!
//! #### Precision and Range Validation
//!
//! All arithmetic operations automatically validate:
//!
//! - **Range Constraints**: Result values must satisfy the target type's bounds
//! - **Overflow Detection**: Operations that may exceed representable ranges return errors
//! - **NaN/Infinity**: Operations producing NaN or infinity return `FloatError::NaN`
//!
//! This ensures mathematical correctness while maintaining ergonomic API design through automatic type inference.
//!
//! ## Comparison Operations
//!
//! All types support full ordering operations:
//!
//! ```
//! use strict_num_extended::PositiveF32;
//!
//! const A: PositiveF32 = PositiveF32::new_const(5.0);
//! const B: PositiveF32 = PositiveF32::new_const(10.0);
//!
//! assert!(A < B);
//! assert!(B > A);
//! assert!(A <= B);
//! assert!(B >= A);
//! assert_ne!(A, B);
//! ```
//!
//! ## Result Type Arithmetic
//!
//! Arithmetic operations between `Result<T, FloatError>` and concrete types are supported
//! with automatic error propagation:
//!
//! ### Result op Concrete
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Result<LHS> op Concrete<RHS>
//! let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new(5.0).unwrap());
//! const B: NegativeF64 = NegativeF64::new_const(-3.0);
//! let result: Result<FinF64, FloatError> = a + B;
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap().get(), 2.0);
//!
//! // Error propagation
//! let err: Result<PositiveF64, FloatError> = Err(FloatError::NaN);
//! let result: Result<FinF64, FloatError> = err + B;
//! assert!(result.is_err());
//! ```
//!
//! ### Concrete op Result
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Concrete<LHS> op Result<RHS>
//! const A: PositiveF64 = PositiveF64::new_const(10.0);
//! let b: Result<NegativeF64, FloatError> = Ok(NegativeF64::new(-3.0).unwrap());
//! let result: Result<FinF64, FloatError> = A + b;
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap().get(), 7.0);
//!
//! // Error on RHS
//! let err_b: Result<NegativeF64, FloatError> = Err(FloatError::PosInf);
//! let result: Result<FinF64, FloatError> = A + err_b;
//! assert!(result.is_err());
//! ```
//!
//! ### Error Propagation Rules
//!
//! - If LHS is `Err`, the error propagates directly
//! - If RHS is `Err`, the error propagates directly
//! - If both are `Ok`, the operation proceeds with normal validation
//! - Division by zero returns `Err(FloatError::NaN)`
//!
//! ```
//! use strict_num_extended::*;
//!
//! // Division by zero detection
//! let a: Result<NonNegativeF64, FloatError> = Ok(NonNegativeF64::new(10.0).unwrap());
//! const ZERO: NonNegativeF64 = NonNegativeF64::new_const(0.0);
//! let result: Result<NonNegativeF64, FloatError> = a / ZERO;
//! assert!(result.is_err());
//! assert_eq!(result.unwrap_err(), FloatError::NaN);
//! ```
//!
//! ## Option Type Arithmetic
//!
//! Arithmetic operations with `Option<T>` types provide graceful handling of optional values:
//!
//! ### Safe Operations (Concrete op Option)
//!
//! Safe operations (like `Positive + Negative`) return `Option<Output>`:
//!
//! ```
//! use strict_num_extended::*;
//!
//! const A: PositiveF64 = PositiveF64::new_const(5.0);
//! let b: Option<NegativeF64> = Some(NegativeF64::new(-3.0).unwrap());
//! let result: Option<FinF64> = A + b;
//! assert!(result.is_some());
//! assert_eq!(result.unwrap().get(), 2.0);
//!
//! // None propagation
//! let none_b: Option<NegativeF64> = None;
//! let result: Option<FinF64> = A + none_b;
//! assert!(result.is_none());
//! ```
//!
//! ### Unsafe Operations (Concrete op Option)
//!
//! Unsafe operations (multiplication, division) return `Result<Output, FloatError>`:
//!
//! ```
//! use strict_num_extended::*;
//!
//! const A: PositiveF64 = PositiveF64::new_const(5.0);
//! let b: Option<PositiveF64> = Some(PositiveF64::new(3.0).unwrap());
//! let result: Result<PositiveF64, FloatError> = A * b;
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap().get(), 15.0);
//!
//! // None operand returns error
//! let none_b: Option<PositiveF64> = None;
//! let result: Result<PositiveF64, FloatError> = A / none_b;
//! assert!(result.is_err());
//! assert_eq!(result.unwrap_err(), FloatError::NoneOperand);
//! ```
//!
//! ### Chaining Option Operations
//!
//! Option arithmetic can be chained for complex calculations:
//!
//! ```
//! use strict_num_extended::*;
//!
//! const A: PositiveF64 = PositiveF64::new_const(10.0);
//! let b: Option<NegativeF64> = Some(NegativeF64::new(-2.0).unwrap());
//! let c: Option<PositiveF64> = Some(PositiveF64::new(3.0).unwrap());
//!
//! // Safe operation returns Option
//! let step1: Option<FinF64> = A + b;  // Some(8.0)
//! assert!(step1.is_some());
//!
//! // Chain with unsafe operation
//! let result: Result<FinF64, FloatError> = step1.map(|x| x * c).unwrap();
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap().get(), 24.0);
//! ```
//!
//! ## Result & Option Arithmetic Overview
//!
//! The library provides comprehensive support for arithmetic operations with `Result<T, FloatError>`
//! and `Option<T>` types:
//!
//! ### Result Types
//!
//! Automatic error propagation for arithmetic operations with `Result<T, FloatError>`:
//!
//! - Operations between `Result<T>` and concrete types automatically propagate errors
//! - If either operand is `Err`, the error is forwarded directly
//! - When both operands are `Ok`, the operation proceeds with normal validation
//! - Division by zero returns `FloatError::NaN`
//!
//! This eliminates verbose error handling boilerplate in calculations that may fail.
//!
//! ### Option Types
//!
//! Graceful handling of optional values in arithmetic operations:
//!
//! - **Safe Operations** (e.g., `Positive + Negative`): Return `Option<Output>`, propagating `None` automatically
//! - **Unsafe Operations** (e.g., multiplication, division): Return `Result<Output, FloatError>`, with `None` operands converted to `FloatError::NoneOperand`
//! - Supports chaining operations for complex calculations with optional values
//!
//! This design provides ergonomic handling of missing values without nested match expressions.
//!
//! > **Note**: For more detailed examples and API documentation, see the specific sections above
//! > covering [Result Type Arithmetic](#result-type-arithmetic) and [Option Type Arithmetic](#option-type-arithmetic).
//!
//! # Error Handling
//!
//! All operations that can fail return `Result<T, FloatError>`. The `FloatError` enum
//! provides detailed error information:
//!
//! ## Error Types
//!
//! - `FloatError::NaN` - Value is NaN (Not a Number)
//! - `FloatError::PosInf` - Value is positive infinity
//! - `FloatError::NegInf` - Value is negative infinity
//! - `FloatError::OutOfRange` - Value is outside the valid range for the target type
//! - `FloatError::NoneOperand` - Right-hand side operand is None in Option arithmetic
//!
//! ## Example: Error Handling
//!
//! ```
//! use strict_num_extended::{FinF32, NonZeroF32, FloatError, NormalizedF32, PositiveF64};
//!
//! // Successful creation
//! let valid: Result<FinF32, FloatError> = FinF32::new(3.14);
//! assert!(valid.is_ok());
//!
//! // NaN error
//! let nan: Result<FinF32, FloatError> = FinF32::new(f32::NAN);
//! assert!(nan.is_err());
//! assert_eq!(nan.unwrap_err(), FloatError::NaN);
//!
//! // Infinity error
//! let inf: Result<FinF32, FloatError> = FinF32::new(f32::INFINITY);
//! assert!(inf.is_err());
//! assert_eq!(inf.unwrap_err(), FloatError::PosInf);
//!
//! // Out of range error
//! let out_of_range: Result<NormalizedF32, FloatError> = NormalizedF32::new(2.0);
//! assert!(out_of_range.is_err());
//! assert_eq!(out_of_range.unwrap_err(), FloatError::OutOfRange);
//!
//! // Division by zero is prevented at creation time
//! let a = PositiveF64::new(10.0).unwrap();
//! let zero_result = PositiveF64::new(0.0);
//! assert!(zero_result.is_err());  // Cannot create zero value
//! assert_eq!(zero_result.unwrap_err(), FloatError::OutOfRange);
//! ```
//!
//! ## Practical Example: Safe Division Function
//!
//! ```
//! use strict_num_extended::{FinF32, NonZeroF32, FloatError};
//!
//! fn safe_divide(
//!     numerator: Result<FinF32, FloatError>,
//!     denominator: Result<NonZeroF32, FloatError>,
//! ) -> Result<FinF32, FloatError> {
//!     match (numerator, denominator) {
//!         (Ok(num), Ok(denom)) => {
//!             // Safe: denom is guaranteed non-zero
//!             FinF32::new(num.get() / denom.get())
//!         }
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
//! // Division by zero is prevented
//! let invalid = safe_divide(
//!     FinF32::new(10.0),
//!     NonZeroF32::new(0.0)  // Returns Err
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
//! ### Unsafe Creation
//!
//! For performance-critical code where you can guarantee validity, use `new_unchecked()`:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // WARNING: Only use when you can guarantee the value is valid!
//! // Undefined behavior if constraint is violated
//! const FIN: FinF32 = unsafe { FinF32::new_unchecked(3.14) };
//! assert_eq!(FIN.get(), 3.14);
//!
//! // Safe usage: compile-time constant
//! const ONE: PositiveF32 = unsafe { PositiveF32::new_unchecked(1.0) };
//!
//! // UNSAFE: Invalid value causes undefined behavior
//! // const NAN: FinF32 = unsafe { FinF32::new_unchecked(f32::NAN) };
//! ```
//!
//! **Note**: Prefer `new_const()` for compile-time constants. It provides validation
//! during compilation and is safer than `new_unchecked()`.
//!
//! # How Constraints Work
//!
//! All types automatically enforce constraints through runtime validation and compile-time checks:
//!
//! - **Compile-time**: Use `new_const()` to create validated constants
//! - **Runtime**: Use `new()` which returns `Result<T, FloatError>` after validation
//!
//! Constraint violations are caught early:
//! - At compile time for constants (if value is invalid)
//! - At runtime for dynamic values (returns `FloatError`)
//!
//! ## Finite Constraint
//!
//! The `Fin` types exclude only NaN and infinity, accepting all other finite values:
//!
//! ```
//! use strict_num_extended::FinF32;
//!
//! let valid = FinF32::new(3.14);         // Ok(value)
//! let invalid = FinF32::new(f32::NAN);    // Err(FloatError::NaN)
//! let invalid = FinF32::new(f32::INFINITY); // Err(FloatError::PosInf)
//! ```
//!
//! ## `NonNegative` Constraint
//!
//! The `NonNegative` types require finite AND non-negative values (x ≥ 0):
//!
//! ```
//! use strict_num_extended::NonNegativeF32;
//!
//! let valid = NonNegativeF32::new(0.0);      // Ok(value)
//! let valid = NonNegativeF32::new(1.5);      // Ok(value)
//! let invalid = NonNegativeF32::new(-1.0);   // Err(FloatError::OutOfRange) (negative)
//! let invalid = NonNegativeF32::new(f32::INFINITY); // Err(FloatError::PosInf) (infinite)
//! ```
//!
//! ## `NonPositive` Constraint
//!
//! The `NonPositive` types require finite AND non-positive values (x ≤ 0):
//!
//! ```
//! use strict_num_extended::NonPositiveF32;
//!
//! let valid = NonPositiveF32::new(0.0);      // Ok(value)
//! let valid = NonPositiveF32::new(-1.5);     // Ok(value)
//! let invalid = NonPositiveF32::new(1.0);    // Err(FloatError::OutOfRange) (positive)
//! let invalid = NonPositiveF32::new(f32::NEG_INFINITY); // Err(FloatError::NegInf) (infinite)
//! ```
//!
//! ## `NonZero` Constraint
//!
//! The `NonZero` types require finite AND non-zero values (x ≠ 0), excluding both +0.0 and -0.0:
//!
//! ```
//! use strict_num_extended::NonZeroF32;
//!
//! let valid = NonZeroF32::new(1.0);       // Ok(value)
//! let valid = NonZeroF32::new(-1.0);      // Ok(value)
//! let invalid = NonZeroF32::new(0.0);     // Err(FloatError::OutOfRange) (zero)
//! let invalid = NonZeroF32::new(-0.0);    // Err(FloatError::OutOfRange) (negative zero)
//! ```
//!
//! ## Positive Constraint
//!
//! The `Positive` types require finite AND positive values (x > 0):
//!
//! ```
//! use strict_num_extended::PositiveF32;
//!
//! let valid = PositiveF32::new(1.0);       // Ok(value)
//! let valid = PositiveF32::new(0.001);     // Ok(value)
//! let invalid = PositiveF32::new(0.0);     // Err(FloatError::OutOfRange) (zero)
//! let invalid = PositiveF32::new(-1.0);    // Err(FloatError::OutOfRange) (negative)
//! let invalid = PositiveF32::new(f32::INFINITY); // Err(FloatError::PosInf) (infinite)
//! ```
//!
//! ## Negative Constraint
//!
//! The `Negative` types require finite AND negative values (x < 0):
//!
//! ```
//! use strict_num_extended::NegativeF32;
//!
//! let valid = NegativeF32::new(-1.0);      // Ok(value)
//! let valid = NegativeF32::new(-0.001);    // Ok(value)
//! let invalid = NegativeF32::new(0.0);     // Err(FloatError::OutOfRange) (zero)
//! let invalid = NegativeF32::new(1.0);     // Err(FloatError::OutOfRange) (positive)
//! let invalid = NegativeF32::new(f32::NEG_INFINITY); // Err(FloatError::NegInf) (infinite)
//! ```
//!
//! ## Normalized Constraint
//!
//! The `Normalized` types require finite values in [0.0, 1.0]:
//!
//! ```
//! use strict_num_extended::NormalizedF32;
//!
//! let valid = NormalizedF32::new(0.75);   // Ok(value)
//! let valid = NormalizedF32::new(0.0);    // Ok(value)
//! let valid = NormalizedF32::new(1.0);    // Ok(value)
//! let invalid = NormalizedF32::new(1.5);  // Err(FloatError::OutOfRange) (> 1.0)
//! let invalid = NormalizedF32::new(-0.5); // Err(FloatError::OutOfRange) (< 0.0)
//! ```
//!
//! ## `NegativeNormalized` Constraint
//!
//! The `NegativeNormalized` types require finite values in [-1.0, 0.0]:
//!
//! ```
//! use strict_num_extended::NegativeNormalizedF32;
//!
//! let valid = NegativeNormalizedF32::new(-0.75);  // Ok(value)
//! let valid = NegativeNormalizedF32::new(-1.0);   // Ok(value)
//! let valid = NegativeNormalizedF32::new(0.0);    // Ok(value)
//! let invalid = NegativeNormalizedF32::new(1.5);  // Err(FloatError::OutOfRange) (> 0.0)
//! let invalid = NegativeNormalizedF32::new(-1.5); // Err(FloatError::OutOfRange) (< -1.0)
//! ```
//!
//! ## `Symmetric` Constraint
//!
//! The `Symmetric` types require finite values in [-1.0, 1.0]:
//!
//! ```
//! use strict_num_extended::SymmetricF32;
//!
//! let valid = SymmetricF32::new(0.75);   // Ok(value)
//! let valid = SymmetricF32::new(-0.5);   // Ok(value)
//! let valid = SymmetricF32::new(1.0);    // Ok(value)
//! let valid = SymmetricF32::new(-1.0);   // Ok(value)
//! let invalid = SymmetricF32::new(1.5);  // Err(FloatError::OutOfRange) (> 1.0)
//! let invalid = SymmetricF32::new(-1.5); // Err(FloatError::OutOfRange) (< -1.0)
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
//! use strict_num_extended::PositiveF32;
//!
//! // Positive = NonNegative AND NonZero (equivalent to x > 0)
//! let valid = PositiveF32::new(1.0);     // Ok(value)
//! let valid = PositiveF32::new(0.001);   // Ok(value)
//! let invalid = PositiveF32::new(0.0);   // Err(FloatError::OutOfRange) (zero)
//! let invalid = PositiveF32::new(-1.0);  // Err(FloatError::OutOfRange) (negative)
//! ```
//!
//! ## Unary Negation
//!
//! The unary negation operator (`-`) is supported with automatic type inference:
//!
//! ```
//! use strict_num_extended::*;
//!
//! // NonNegative ↔ NonPositive
//! const NON_NEG: NonNegativeF64 = NonNegativeF64::new_const(5.0);
//! let non_pos: NonPositiveF64 = -NON_NEG;
//! assert_eq!(non_pos.get(), -5.0);
//!
//! // Positive ↔ Negative
//! const POS: PositiveF32 = PositiveF32::new_const(10.0);
//! let neg: NegativeF32 = -POS;
//! assert_eq!(neg.get(), -10.0);
//!
//! // Normalized ↔ NegativeNormalized
//! const NORM: NormalizedF64 = NormalizedF64::new_const(0.75);
//! let neg_norm: NegativeNormalizedF64 = -NORM;
//! assert_eq!(neg_norm.get(), -0.75);
//!
//! // Fin is reflexive (negating Fin returns Fin)
//! const FIN: FinF32 = FinF32::new_const(2.5);
//! let neg_fin: FinF32 = -FIN;
//! assert_eq!(neg_fin.get(), -2.5);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::manual_range_contains)]

#[cfg(feature = "std")]
extern crate std;

// Generate all code using proc_macro
strict_num_extended_macros::generate_finite_float_types!([
    (Fin, []),
    (NonNegative, [">= 0.0"]),
    (NonPositive, ["<= 0.0"]),
    (NonZero, ["!= 0.0"]),
    (Normalized, [">= 0.0", "<= 1.0"]),
    (NegativeNormalized, [">= -1.0", "<= 0.0"]),
    (Positive, ["> 0.0"]),
    (Negative, ["< 0.0"]),
    (Symmetric, [">= -1.0", "<= 1.0"]),
]);
