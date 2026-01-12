//! Documentation generation helper functions module
//!
//! Intelligently generates type and method documentation comments based on constraint definitions

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::config::{Bounds, ConstraintDef, Sign};
use core::f64::consts::PI;

/// Generates documentation comments for struct definitions
///
/// Creates complete documentation with constraint descriptions, mathematical formulas, and examples for each generated struct
pub fn generate_struct_doc(
    type_name: &Ident,
    float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> TokenStream {
    let float_bits = if float_type == "f32" { "32" } else { "64" };
    let type_name_str = type_name.to_string();
    let struct_name = format!("{}{}", type_name, float_type.to_string().to_uppercase());

    // Generate constraint description
    let constraint_desc = generate_constraint_description(constraint_def);
    let constraint_formula = generate_constraint_formula(constraint_def);

    // Generate different descriptions based on type
    let type_description =
        generate_type_description(&struct_name, type_name, float_type, constraint_def);

    quote! {
        concat!("A ", #float_bits, "-bit floating-point number representing a **", #type_name_str, "** value.\n\n",
               "# Constraints\n\n",
               "This type enforces the following constraints:\n",
               "- **Range**: `", #constraint_formula, "` (", #constraint_desc, ")\n",
               "- **Finite**: Excludes NaN and ±∞\n\n",
               #type_description)
    }
}

/// Generates a mathematical formula expression for constraints
pub fn generate_constraint_formula(constraint_def: &ConstraintDef) -> String {
    match (
        constraint_def.sign,
        constraint_def.excludes_zero,
        &constraint_def.bounds,
    ) {
        // Positive types
        (
            Sign::Positive,
            false,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => "x ≥ 0".to_string(),
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => "x > 0".to_string(),
        (
            Sign::Positive,
            _,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            if constraint_def.excludes_zero {
                format!(
                    "{} < x ≤ {}",
                    format_bound_value((*l).max(0.0)),
                    format_bound_value(*u)
                )
            } else {
                format!(
                    "{} ≤ x ≤ {}",
                    format_bound_value((*l).max(0.0)),
                    format_bound_value(*u)
                )
            }
        }

        // Negative types
        (
            Sign::Negative,
            false,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => "x ≤ 0".to_string(),
        (
            Sign::Negative,
            true,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => "x < 0".to_string(),
        (
            Sign::Negative,
            _,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            if constraint_def.excludes_zero {
                format!(
                    "{} ≤ x < {}",
                    format_bound_value(*l),
                    format_bound_value((*u).min(0.0))
                )
            } else {
                format!(
                    "{} ≤ x ≤ {}",
                    format_bound_value(*l),
                    format_bound_value((*u).min(0.0))
                )
            }
        }

        // NonZero types
        (
            Sign::Any,
            true,
            Bounds {
                lower: None,
                upper: None,
            },
        ) => "x ≠ 0".to_string(),

        // Bounded types
        (
            Sign::Any,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            // Special formatting for PI bounds
            let lower_str = format_bound_value(*l);
            let upper_str = format_bound_value(*u);
            format!("{} ≤ x ≤ {}", lower_str, upper_str)
        }
        (
            Sign::Any,
            true,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            if *l == 0.0 {
                format!("0 < x ≤ {}", u)
            } else if *u == 0.0 {
                format!("{} ≤ x < 0", l)
            } else {
                format!("{} ≤ x ≤ {}, x ≠ 0", l, u)
            }
        }

        // Default: finite numbers
        _ => "x ∈ ℝ".to_string(),
    }
}

/// Generates a text description of constraints
pub fn generate_constraint_description(constraint_def: &ConstraintDef) -> String {
    match (
        constraint_def.sign,
        constraint_def.excludes_zero,
        &constraint_def.bounds,
    ) {
        // Positive types
        (
            Sign::Positive,
            false,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => "non-negative".to_string(),
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => "positive".to_string(),
        (
            Sign::Positive,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!(
                "range [{}, {}]",
                format_bound_value((*l).max(0.0)),
                format_bound_value(*u)
            )
        }
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!(
                "range ({}, {}]",
                format_bound_value((*l).max(0.0)),
                format_bound_value(*u)
            )
        }

        // Negative types
        (
            Sign::Negative,
            false,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => "non-positive".to_string(),
        (
            Sign::Negative,
            true,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => "negative".to_string(),
        (
            Sign::Negative,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!(
                "range [{}, {}]",
                format_bound_value(*l),
                format_bound_value((*u).min(0.0))
            )
        }
        (
            Sign::Negative,
            true,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!(
                "range [{}, {})",
                format_bound_value(*l),
                format_bound_value((*u).min(0.0))
            )
        }

        // NonZero types
        (
            Sign::Any,
            true,
            Bounds {
                lower: None,
                upper: None,
            },
        ) => "non-zero".to_string(),

        // Bounded types
        (
            Sign::Any,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!(
                "range [{}, {}]",
                format_bound_value(*l),
                format_bound_value(*u)
            )
        }
        (
            Sign::Any,
            true,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            if *l == 0.0 {
                format!("range (0, {}]", format_bound_value(*u))
            } else if *u == 0.0 {
                format!("range [{}, 0)", format_bound_value(*l))
            } else {
                format!(
                    "range [{}, {}], excluding zero",
                    format_bound_value(*l),
                    format_bound_value(*u)
                )
            }
        }

        // Default: finite numbers
        _ => "finite".to_string(),
    }
}

/// Generates additional description information for types
fn generate_type_description(
    struct_name: &str,
    type_name: &Ident,
    _float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> String {
    use crate::config::Sign;

    let bounds = &constraint_def.bounds;
    let sign = constraint_def.sign;
    let excludes_zero = constraint_def.excludes_zero;

    // Match based on constraint properties, not type names
    match (sign, excludes_zero, bounds) {
        // Negative normalized [-1, 0]
        (Sign::Negative, _, b) if b.is_negative_normalized() => format!(
            "Negative normalized numbers in [-1, 0] are commonly used for:\n\
             - Negative probabilities and offsets\n\
             - Descending normalized values\n\
             \n\
             # Examples\n\
             \n\
             Creating a negative normalized value:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let neg_norm = {0}::new(-0.5)?;\n\
             assert_eq!(neg_norm.get(), -0.5);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (out of range):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(0.5);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        // Normalized [0, 1]
        (Sign::Positive, false, b) if b.is_normalized() => format!(
            "Normalized numbers in [0, 1] are commonly used for:\n\
             - Probabilities and percentages\n\
             - Color channel values (RGB/RGBA)\n\
             - Neural network activations\n\
             - Normalized coordinates\n\
             \n\
             # Examples\n\
             \n\
             Creating a normalized value:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let norm = {0}::new(0.75)?;\n\
             assert_eq!(norm.get(), 0.75);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (out of range):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(1.5);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        // Symmetric [-1, 1]
        (Sign::Any, _, b) if b.is_symmetric() => format!(
            "Symmetric numbers in [-1, 1] are commonly used for:\n\
             - Coordinates and offsets\n\
             - Differences and deltas\n\
             - Normalized symmetric ranges\n\
             \n\
             # Examples\n\
             \n\
             Creating a symmetric value:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let sym = {0}::new(0.5)?;\n\
             assert_eq!(sym.get(), 0.5);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n",
            struct_name
        ),

        // Unbounded positive (excludes_zero: true => Positive)
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => format!(
            "Positive numbers are useful for:\n\
             - Strictly positive values\n\
             - Non-zero multipliers\n\
             \n\
             # Examples\n\
             \n\
             Creating a non-zero positive value:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let pos = {0}::new(42.0)?;\n\
             assert_eq!(pos.get(), 42.0);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (zero):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(0.0);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        // Unbounded positive (excludes_zero: false => NonNegative)
        (
            Sign::Positive,
            false,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => format!(
            "Non-negative numbers are commonly used for:\n\
             - Counting and magnitudes\n\
             - Physical measurements\n\
             - Financial values\n\
             \n\
             # Examples\n\
             \n\
             Creating a non-negative number:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let pos = {0}::new(42.0)?;\n\
             assert_eq!(pos.get(), 42.0);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (negative):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(-1.0);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        // Unbounded negative (excludes_zero: true => Negative)
        (
            Sign::Negative,
            true,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => format!(
            "Negative numbers are useful for:\n\
             - Strictly negative values\n\
             - Non-zero denominators\n\
             \n\
             # Examples\n\
             \n\
             Creating a negative value:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let neg = {0}::new(-42.0)?;\n\
             assert_eq!(neg.get(), -42.0);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (zero):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(0.0);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        // Unbounded negative (excludes_zero: false => NonPositive)
        (
            Sign::Negative,
            false,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => format!(
            "Non-positive numbers are commonly used for:\n\
             - Losses and debts\n\
             - Temperature below zero\n\
             - Directions and offsets\n\
             \n\
             # Examples\n\
             \n\
             Creating a non-positive number:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let neg = {0}::new(-42.0)?;\n\
             assert_eq!(neg.get(), -42.0);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (positive):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(1.0);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        // NonZero (unbounded, excludes_zero: true)
        (
            Sign::Any,
            true,
            Bounds {
                lower: None,
                upper: None,
            },
        ) => format!(
            "Non-zero numbers are useful for:\n\
             - Division operations (avoiding divide-by-zero)\n\
             - Multiplicative factors\n\
             - Scaling operations\n\
             \n\
             # Examples\n\
             \n\
             Creating a non-zero value:\n\
             \n\
             ```rust\n\
             #![expect(clippy::approx_constant)]\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let nonzero = {0}::new(42.0)?;\n\
             assert_eq!(nonzero.get(), 42.0);\n\
             # Ok::<(), FloatError>(())\n\
             ```\n\
             \n\
             Invalid value (zero):\n\
             \n\
             ```rust\n\
             use strict_num_extended::{0};\n\
             \n\
             let invalid = {0}::new(0.0);\n\
             assert!(invalid.is_err());\n\
             ```\n",
            struct_name
        ),

        _ => {
            // Default description for other types
            let valid_example = generate_valid_example_for_type(type_name, constraint_def);
            format!(
                "# Examples\n\n\
                 Creating a value:\n\n\
                 ```rust\n\
                 #![expect(clippy::approx_constant)]\n\
                 use strict_num_extended::{{{0}, FloatError}};\n\n\
                 let value = {0}::new({1})?;\n\
                 assert_eq!(value.get(), {1});\n\
                 # Ok::<(), FloatError>(())\n\
                 ```\n",
                struct_name, valid_example
            )
        }
    }
}

/// Generates a valid example value for a specific type
fn generate_valid_example_for_type(_type_name: &Ident, constraint_def: &ConstraintDef) -> String {
    use crate::config::Sign;

    match (&constraint_def.bounds, constraint_def.sign) {
        // Unbounded positive
        (
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
            Sign::Positive,
        ) => "42.0".to_string(),

        // Unbounded negative
        (
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
            Sign::Negative,
        ) => "-42.0".to_string(),

        // Normalized [0, 1]
        (b, _) if b.is_normalized() => "0.5".to_string(),

        // Symmetric [-1, 1]
        (b, _) if b.is_symmetric() => "0.0".to_string(),

        // Negative normalized [-1, 0]
        (b, _) if b.is_negative_normalized() => "-0.5".to_string(),

        // Bounded types - return midpoint
        (
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
            _,
        ) => {
            let mid = (l + u) / 2.0;
            if mid.abs() < 1e-10 {
                // Midpoint is zero, but if zero is excluded, use offset value
                if constraint_def.excludes_zero {
                    format!("{}", l + (u - l) * 0.25)
                } else {
                    "0.0".to_string()
                }
            } else {
                format!("{}", mid)
            }
        }

        // Default
        _ => "1.0".to_string(),
    }
}

/// Generates documentation for the `new()` method
pub fn generate_new_method_doc(
    struct_name: &Ident,
    float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> TokenStream {
    let type_name_str = struct_name.to_string();
    let constraint_desc = generate_constraint_description(constraint_def);

    let valid_example = generate_valid_example(float_type, constraint_def);
    let (invalid_example, invalid_reason) = generate_invalid_example(float_type, constraint_def);

    quote! {
        concat!("Creates a new ", #type_name_str, " value.\n\n",
               "The value must satisfy the constraint: ", #constraint_desc, ".\n\n",
               "# Examples\n\n",
               "Valid value:\n\n",
               "```rust\n",
               "#![expect(clippy::approx_constant)]\n",
               "use strict_num_extended::{", #type_name_str, ", FloatError};\n\n",
               "let value = ", #type_name_str, "::new(", #valid_example, ")?;\n",
               "assert_eq!(value.get(), ", #valid_example, ");\n",
               "# Ok::<(), FloatError>(())\n",
               "```\n\n",
               "Invalid value (", #invalid_reason, "):\n\n",
               "```rust\n",
               "use strict_num_extended::", #type_name_str, ";\n\n",
               "let invalid = ", #type_name_str, "::new(", #invalid_example, ");\n",
               "assert!(invalid.is_err());\n",
               "```\n\n",
               "# Errors\n\n",
               "Returns `Err(FloatError)` if the value does not satisfy the constraint.")
    }
}

/// Generates a valid example value
pub fn generate_valid_example(_float_type: &Ident, constraint_def: &ConstraintDef) -> String {
    match (constraint_def.sign, &constraint_def.bounds) {
        (
            Sign::Positive,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => "42.0".to_string(),
        (
            Sign::Positive,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        )
        | (
            Sign::Negative,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("{}", (*l + *u) / 2.0)
        }
        (
            Sign::Negative,
            Bounds {
                upper: Some(0.0),
                lower: None,
            },
        ) => "-42.0".to_string(),
        (
            Sign::Any,
            Bounds {
                lower: Some(0.0),
                upper: Some(1.0),
            },
        ) => "0.5".to_string(),
        (
            Sign::Any,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            let mid = (*l + *u) / 2.0;
            if mid != 0.0 {
                format!("{}", mid)
            } else {
                format!("{}", *l + 0.1)
            }
        }
        _ => "1.0".to_string(),
    }
}

/// Generates an invalid example value (for demonstrating errors)
pub fn generate_invalid_example(
    float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> (String, &'static str) {
    match (
        constraint_def.sign,
        constraint_def.excludes_zero,
        &constraint_def.bounds,
    ) {
        (
            Sign::Positive,
            _,
            Bounds {
                lower: Some(0.0), ..
            },
        ) => ("-1.0".to_string(), "negative value"),
        (
            Sign::Negative,
            _,
            Bounds {
                upper: Some(0.0), ..
            },
        ) => ("1.0".to_string(), "positive value"),
        (
            Sign::Any,
            true,
            Bounds {
                lower: None,
                upper: None,
            },
        ) => ("0.0".to_string(), "zero value"),
        (Sign::Any, false, Bounds { lower: Some(l), .. }) if *l > 0.0 => {
            (format!("{}", *l - 1.0), "out of lower bound")
        }
        (Sign::Any, false, Bounds { upper: Some(u), .. }) if *u < 0.0 => {
            (format!("{}", *u + 1.0), "out of upper bound")
        }
        _ => (
            format!("{}::NAN", float_type.to_string().to_lowercase()),
            "NaN",
        ),
    }
}

/// Formats a bound value for display in documentation
/// Special handling for PI to avoid `clippy::approx_constant` warnings
fn format_bound_value(value: f64) -> String {
    const PI_TOLERANCE: f64 = 0.00001;

    if (value - PI).abs() < PI_TOLERANCE {
        "PI".to_string()
    } else if (value + PI).abs() < PI_TOLERANCE {
        "-PI".to_string()
    } else if (value - PI / 2.0).abs() < PI_TOLERANCE {
        "PI/2".to_string()
    } else if (value + PI / 2.0).abs() < PI_TOLERANCE {
        "-PI/2".to_string()
    } else if (value - PI / 3.0).abs() < PI_TOLERANCE {
        "PI/3".to_string()
    } else if (value + PI / 3.0).abs() < PI_TOLERANCE {
        "-PI/3".to_string()
    } else if (value - PI / 4.0).abs() < PI_TOLERANCE {
        "PI/4".to_string()
    } else if (value + PI / 4.0).abs() < PI_TOLERANCE {
        "-PI/4".to_string()
    } else if (value - PI / 6.0).abs() < PI_TOLERANCE {
        "PI/6".to_string()
    } else if (value + PI / 6.0).abs() < PI_TOLERANCE {
        "-PI/6".to_string()
    } else if (value - PI / 8.0).abs() < PI_TOLERANCE {
        "PI/8".to_string()
    } else if (value + PI / 8.0).abs() < PI_TOLERANCE {
        "-PI/8".to_string()
    } else if (value - 1.0 / PI).abs() < PI_TOLERANCE {
        "1/PI".to_string()
    } else if (value + 1.0 / PI).abs() < PI_TOLERANCE {
        "-1/PI".to_string()
    } else if (value - 2.0 / PI).abs() < PI_TOLERANCE {
        "2/PI".to_string()
    } else if (value + 2.0 / PI).abs() < PI_TOLERANCE {
        "-2/PI".to_string()
    } else {
        format!("{}", value)
    }
}
