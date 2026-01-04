# strict-num-extended

[![Crates.io](https://img.shields.io/crates/v/strict-num-extended)](https://crates.io/crates/strict-num-extended)
[![Documentation](https://docs.rs/strict-num-extended/badge.svg)](https://docs.rs/strict-num-extended)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust Edition: 2024](https://img.shields.io/badge/Rust-Edition%202024-orange.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)

**Type-safe finite floating-point numbers for Rust**

A Rust library providing zero-cost finite floating-point types through the type system, guaranteeing safety by eliminating NaN and infinity values at compile time.

## Features

- **Type Safety** - Catch floating-point errors at compile time, not runtime
- **Zero-Cost Abstractions** - No performance overhead compared to built-in types
- **Comprehensive Constraints** - Finite, Positive, Negative, NonZero, Normalized, NegativeNormalized
- **Composable Constraints** - Combine multiple constraints (e.g., NonZeroPositive)
- **Full Trait Implementation** - PartialEq, Eq, PartialOrd, Ord, Display, Debug, arithmetic operators
- **Const Support** - Create compile-time constants with validation
- **Option Types** - Optional variants for graceful error handling

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
strict-num-extended = "0.1.1"
```

### Basic Usage

```rust
use strict_num_extended::{FinF32, PositiveF32, NonZeroPositiveF32};

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
```

> **Note**: For more detailed examples, constraint explanations, and API documentation, see the [module documentation](https://docs.rs/strict-num-extended).

## Available Types

### Single Constraint Types

| Type | Constraint | Valid Range | Example |
|------|------------|-------------|---------|
| `FinF32` / `FinF64` | Finite | All real numbers | `-∞ < x < ∞` |
| `PositiveF32` / `PositiveF64` | Positive | `x ≥ 0` | `0.0, 1.5, 100.0` |
| `NegativeF32` / `NegativeF64` | Negative | `x ≤ 0` | `0.0, -1.5, -100.0` |
| `NonZeroF32` / `NonZeroF64` | NonZero | `x ≠ 0` | `1.0, -1.0, 0.001` |
| `NormalizedF32` / `NormalizedF64` | Normalized | `0.0 ≤ x ≤ 1.0` | `0.0, 0.5, 1.0` |
| `NegativeNormalizedF32` / `NegativeNormalizedF64` | Negative Normalized | `-1.0 ≤ x ≤ 0.0` | `-0.5, -0.75, -1.0` |

### Combined Constraint Types

| Type | Constraints | Valid Range | Example |
|------|-------------|-------------|---------|
| `NonZeroPositiveF32` / `NonZeroPositiveF64` | Positive + NonZero | `x > 0` | `0.001, 1.0, 100.0` |
| `NonZeroNegativeF32` / `NonZeroNegativeF64` | Negative + NonZero | `x < 0` | `-0.001, -1.0, -100.0` |

For detailed constraint validation rules and advanced usage patterns, see the [complete documentation](https://docs.rs/strict-num-extended).

## Development

### Building

```bash
# Build the library
cargo build

# Run tests
cargo test

# Run linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ for type-safe Rust programming**
