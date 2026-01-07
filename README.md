# strict-num-extended

[![Crates.io](https://img.shields.io/crates/v/strict-num-extended)](https://crates.io/crates/strict-num-extended)
[![Documentation](https://docs.rs/strict-num-extended/badge.svg)](https://docs.rs/strict-num-extended)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Type-safe finite floating-point numbers for Rust.

A Rust library providing zero-cost finite floating-point types through the type system, guaranteeing safety by eliminating NaN and infinity values at compile time or runtime.

## Features

- **Type Safety** - Catch floating-point errors at compile time or runtime
- **Zero-Cost Abstractions** - No performance overhead compared to built-in types except the additional value checking
- **Comprehensive Constraints** - Finite, Positive, Negative, NonZero, Normalized, NegativeNormalized, Symmetric, ...
- **Full Trait Implementation** - PartialEq, Eq, PartialOrd, Ord, Display, Debug, arithmetic operators
- **Const Support** - Create compile-time constants with validation
- **Option/Result Arithmetic** - Optional and fallible operations with automatic error propagation
- **Type Conversions** - Safe F32↔F64 conversions with precision detection

## Type Safety

This library provides type safety through both compile-time and runtime guarantees:

### Compile-Time Safety

Create constants at compile time with guaranteed validity using `new_const()`:

```rust
use strict_num_extended::*;

const MAX_VALUE: PositiveF64 = PositiveF64::new_const(100.0);
const HALF: NormalizedF32 = NormalizedF32::new_const(0.5);
```

The `new_const()` method validates constraints at compile time, ensuring invalid values are caught before your code even runs.

### Runtime Safety

At runtime, all operations automatically:

- **Validate value ranges** when creating instances
- **Detect overflow** in arithmetic operations
- **Return detailed errors** via `Result<T, FloatError>` for any violation

```rust
use strict_num_extended::*;

// Value creation validates ranges
let value = PositiveF64::new(42.0);
assert!(value.is_ok());

// Invalid value returns error
let invalid = PositiveF64::new(-1.0);
assert!(invalid.is_err());

// Arithmetic detects overflow
let a = PositiveF64::new(1e308).unwrap();
let b = PositiveF64::new(1e308).unwrap();
let result = a + b;  // Returns Result, detects overflow
assert!(result.is_err());
```

This two-layer safety approach ensures your floating-point code is correct both at compile time and at runtime, catching errors early and preventing undefined behavior.

### Basic Usage

```rust
use strict_num_extended::{FinF32, PositiveF32, NonZeroPositiveF32, SymmetricF32};

// Create finite floating-point numbers (no NaN or infinity)
const FINITE: FinF32 = FinF32::new_const(3.14);
assert_eq!(FINITE.get(), 3.14);

// Rejected: NaN and infinity are not allowed
assert!(FinF32::new(f32::NAN).is_err());
assert!(FinF32::new(f32::INFINITY).is_err());

// Positive numbers (>= 0)
const POSITIVE: PositiveF32 = PositiveF32::new_const(42.0);
assert!(POSITIVE >= PositiveF32::new_const(0.0));

// Non-zero positive numbers (> 0)
const NONZERO_POS: NonZeroPositiveF32 = NonZeroPositiveF32::new_const(10.0);
assert!(NONZERO_POS.get() > 0.0);

// Arithmetic operations preserve constraints
let result = NONZERO_POS + NONZERO_POS;
assert_eq!(result.get(), 20.0);

// Symmetric numbers in range [-1.0, 1.0]
const SYMMETRIC: SymmetricF32 = SymmetricF32::new_const(0.75);
assert_eq!(SYMMETRIC.get(), 0.75);

// Negation is reflexive (Symmetric → Symmetric)
let negated: SymmetricF32 = -SYMMETRIC;
assert_eq!(negated.get(), -0.75);
```

## Type Conversions

### Between Constraints

The library provides seamless conversions between constraint types using standard Rust `From` and `TryFrom` traits:

- **Subset to Superset**: Converting from more constrained types (e.g., `NormalizedF32`) to less constrained types (e.g., `FinF32`) uses the `From` trait and always succeeds
- **Superset to Subset**: Converting from less constrained types to more constrained types uses the `TryFrom` trait and validates the value, returning `Result<T, FloatError>` to handle potential range violations

This design ensures type safety while providing flexibility in working with different constraint levels.

### F32 ↔ F64 Conversions

Safe conversions between F32 and F64 variants with precision awareness:

- **F32 → F64**: Lossless conversion using the `From` trait, always succeeds
- **F64 → F32**: Precision-aware conversion using `TryFrom` trait, detects both range overflow and precision loss
- **Specialized Methods**: The `try_into_f32()` and `as_f64()` methods provide explicit control over F32/F64 conversions with compile-time support

## Result & Option Arithmetic

### Result Types

Automatic error propagation for arithmetic operations with `Result<T, FloatError>`:

- Operations between `Result<T>` and concrete types automatically propagate errors
- If either operand is `Err`, the error is forwarded directly
- When both operands are `Ok`, the operation proceeds with normal validation
- Division by zero is detected and returns `FloatError::DivisionByZero`

This eliminates verbose error handling boilerplate in calculations that may fail.

### Option Types

Graceful handling of optional values in arithmetic operations:

- **Safe Operations** (e.g., `Positive + Negative`): Return `Option<Output>`, propagating `None` automatically
- **Unsafe Operations** (e.g., multiplication, division): Return `Result<Output, FloatError>`, with `None` operands converted to `FloatError::NoneOperand`
- Supports chaining operations for complex calculations with optional values

This design provides ergonomic handling of missing values without nested match expressions.

> **Note**: For more detailed examples, and API documentation, see the [module documentation](https://docs.rs/strict-num-extended).

## Available Types

| Type | Valid Range | Example |
|------|-------------|---------|
| `FinF32` / `FinF64` | All real numbers | `-∞ < x < ∞` |
| `PositiveF32` / `PositiveF64` | `x ≥ 0` | `0.0, 1.5, 100.0` |
| `NegativeF32` / `NegativeF64` | `x ≤ 0` | `0.0, -1.5, -100.0` |
| `NonZeroF32` / `NonZeroF64` | `x ≠ 0` | `1.0, -1.0, 0.001` |
| `NormalizedF32` / `NormalizedF64` | `0.0 ≤ x ≤ 1.0` | `0.0, 0.5, 1.0` |
| `NegativeNormalizedF32` / `NegativeNormalizedF64` | `-1.0 ≤ x ≤ 0.0` | `-0.5, -0.75, -1.0` |
| `SymmetricF32` / `SymmetricF64` | `-1.0 ≤ x ≤ 1.0` | `-1.0, 0.0, 0.5, 1.0` |
| `NonZeroPositiveF32` / `NonZeroPositiveF64` | `x > 0` | `0.001, 1.0, 100.0` |
| `NonZeroNegativeF32` / `NonZeroNegativeF64` | `x < 0` | `-0.001, -1.0, -100.0` |

## Error Handling

All fallible operations return `Result<T, FloatError>` with detailed error information:

**Error Types**:
- `NaN` - Value is Not a Number
- `PosInf` - Value is positive infinity
- `NegInf` - Value is negative infinity
- `OutOfRange` - Value is outside the valid range for the target type
- `DivisionByZero` - Division by zero occurred
- `NoneOperand` - Right-hand side operand is None in Option arithmetic

The `FloatError` enum provides comprehensive error information for proper error handling and debugging, allowing precise error matching and recovery strategies.

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
