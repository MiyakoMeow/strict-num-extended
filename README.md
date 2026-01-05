# strict-num-extended

[![Crates.io](https://img.shields.io/crates/v/strict-num-extended)](https://crates.io/crates/strict-num-extended)
[![Documentation](https://docs.rs/strict-num-extended/badge.svg)](https://docs.rs/strict-num-extended)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

Type-safe finite floating-point numbers for Rust.

A Rust library providing zero-cost finite floating-point types through the type system, guaranteeing safety by eliminating NaN and infinity values at compile time.

## Features

- **Type Safety** - Catch floating-point errors at compile time, not runtime
- **Zero-Cost Abstractions** - No performance overhead compared to built-in types
- **Comprehensive Constraints** - Finite, Positive, Negative, NonZero, Normalized, NegativeNormalized, Symmetric
- **Composable Constraints** - Combine multiple constraints (e.g., NonZeroPositive)
- **Full Trait Implementation** - PartialEq, Eq, PartialOrd, Ord, Display, Debug, arithmetic operators
- **Const Support** - Create compile-time constants with validation
- **Option Types** - Optional variants for graceful error handling

### Basic Usage

```rust
use strict_num_extended::{FinF32, PositiveF32, NonZeroPositiveF32, SymmetricF32};

// Create finite floating-point numbers (no NaN or infinity)
let finite = FinF32::new(3.14).unwrap();
assert_eq!(finite.get(), 3.14);

// Rejected: NaN and infinity are not allowed
assert!(FinF32::new(f32::NAN).is_none());
assert!(FinF32::new(f32::INFINITY).is_none());

// Positive numbers (>= 0)
let positive = PositiveF32::new(42.0).unwrap();
assert!(positive >= PositiveF32::new(0.0).unwrap());

// Non-zero positive numbers (> 0)
let nonzero_pos = NonZeroPositiveF32::new(10.0).unwrap();
assert!(nonzero_pos.get() > 0.0);

// Arithmetic operations preserve constraints
let result = nonzero_pos + nonzero_pos;
assert_eq!(result.get(), 20.0);

// Symmetric numbers in range [-1.0, 1.0]
let symmetric = SymmetricF32::new(0.75).unwrap();
assert_eq!(symmetric.get(), 0.75);

// Negation is reflexive (Symmetric → Symmetric)
let negated: SymmetricF32 = -symmetric;
assert_eq!(negated.get(), -0.75);
```

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

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
