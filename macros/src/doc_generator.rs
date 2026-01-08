//! 文档生成辅助函数模块
//!
//! 根据约束定义智能生成类型和方法的文档注释

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::config::{Bounds, ConstraintDef, Sign};

/// 生成结构体定义的文档注释
///
/// 为每个生成的结构体创建包含约束说明、数学公式和示例的完整文档
pub fn generate_struct_doc(
    type_name: &Ident,
    float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> TokenStream {
    let float_bits = if float_type == "f32" { "32" } else { "64" };
    let type_name_str = type_name.to_string();
    let struct_name = format!("{}{}", type_name, float_type.to_string().to_uppercase());

    // 生成约束描述
    let constraint_desc = generate_constraint_description(constraint_def);
    let constraint_formula = generate_constraint_formula(constraint_def);

    // 根据类型生成不同的描述
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

/// 生成约束的数学公式表达式
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
                format!("{} < x ≤ {}", (*l).max(0.0), u)
            } else {
                format!("{} ≤ x ≤ {}", (*l).max(0.0), u)
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
                format!("{} ≤ x < {}", l, (*u).min(0.0))
            } else {
                format!("{} ≤ x ≤ {}", l, (*u).min(0.0))
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
            format!("{} ≤ x ≤ {}", l, u)
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

/// 生成约束的文字描述
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
        ) => "非负数".to_string(),
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(0.0),
                upper: None,
            },
        ) => "正数".to_string(),
        (
            Sign::Positive,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("范围 [{}, {}]", (*l).max(0.0), u)
        }
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("范围 ({}, {}]", (*l).max(0.0), u)
        }

        // Negative types
        (
            Sign::Negative,
            false,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => "非正数".to_string(),
        (
            Sign::Negative,
            true,
            Bounds {
                lower: None,
                upper: Some(0.0),
            },
        ) => "负数".to_string(),
        (
            Sign::Negative,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("范围 [{}, {}]", l, (*u).min(0.0))
        }
        (
            Sign::Negative,
            true,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("范围 [{}, {})", l, (*u).min(0.0))
        }

        // NonZero types
        (
            Sign::Any,
            true,
            Bounds {
                lower: None,
                upper: None,
            },
        ) => "非零".to_string(),

        // Bounded types
        (
            Sign::Any,
            false,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("范围 [{}, {}]", l, u)
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
                format!("范围 (0, {}]", u)
            } else if *u == 0.0 {
                format!("范围 [{}, 0)", l)
            } else {
                format!("范围 [{}, {}], 排除零", l, u)
            }
        }

        // Default: finite numbers
        _ => "有限数".to_string(),
    }
}

/// 生成类型的额外描述信息
fn generate_type_description(
    struct_name: &str,
    type_name: &Ident,
    _float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> String {
    let type_str = type_name.to_string();

    match type_str.as_str() {
        "NegativeNormalized" => format!(
            "Negative normalized numbers in [-1, 0] are commonly used for:\n\
             - Negative probabilities and offsets\n\
             - Descending normalized values\n\
             \n\
             # Examples\n\
             \n\
             Creating a negative normalized value:\n\
             \n\
             ```rust\n\
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

        "NonZeroNegative" => format!(
            "Non-zero negative numbers are useful for:\n\
             - Strictly negative values\n\
             - Non-zero denominators\n\
             \n\
             # Examples\n\
             \n\
             Creating a non-zero negative value:\n\
             \n\
             ```rust\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let neg = {0}::new(-3.14)?;\n\
             assert_eq!(neg.get(), -3.14);\n\
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

        "NonZeroPositive" => format!(
            "Non-zero positive numbers are useful for:\n\
             - Strictly positive values\n\
             - Non-zero multipliers\n\
             \n\
             # Examples\n\
             \n\
             Creating a non-zero positive value:\n\
             \n\
             ```rust\n\
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let pos = {0}::new(3.14)?;\n\
             assert_eq!(pos.get(), 3.14);\n\
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

        "Positive" => format!(
            "Positive numbers are commonly used for:\n\
             - Counting and magnitudes\n\
             - Physical measurements\n\
             - Financial values\n\
             \n\
             # Examples\n\
             \n\
             Creating a positive number:\n\
             \n\
             ```rust\n\
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

        "Negative" => format!(
            "Negative numbers are commonly used for:\n\
             - Losses and debts\n\
             - Temperature below zero\n\
             - Directions and offsets\n\
             \n\
             # Examples\n\
             \n\
             Creating a negative number:\n\
             \n\
             ```rust\n\
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

        "NonZero" => format!(
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
             use strict_num_extended::{{{0}, FloatError}};\n\
             \n\
             let nonzero = {0}::new(3.14)?;\n\
             assert_eq!(nonzero.get(), 3.14);\n\
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

        "Normalized" | "Symmetric" => {
            if type_str == "Normalized" {
                format!(
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
                )
            } else {
                format!(
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
                     use strict_num_extended::{{{0}, FloatError}};\n\
                     \n\
                     let sym = {0}::new(0.5)?;\n\
                     assert_eq!(sym.get(), 0.5);\n\
                     # Ok::<(), FloatError>(())\n\
                     ```\n",
                    struct_name
                )
            }
        }

        _ => {
            // Default example for other types
            let valid_example = generate_valid_example_for_type(type_name, constraint_def);
            format!(
                "# Examples\n\n\
                 Creating a value:\n\n\
                 ```rust\n\
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

/// 生成有效的示例值
fn generate_valid_example_for_type(type_name: &Ident, constraint_def: &ConstraintDef) -> String {
    let type_str = type_name.to_string();

    match type_str.as_str() {
        "Positive" => "42.0".to_string(),
        "Negative" => "-42.0".to_string(),
        "NonZero" => "3.14".to_string(),
        "Normalized" => "0.5".to_string(),
        "Symmetric" => "0.0".to_string(),
        "NonNegative" => "10.0".to_string(),
        "NonPositive" => "-10.0".to_string(),
        "Bounded" => {
            if let (Some(l), Some(u)) = (constraint_def.bounds.lower, constraint_def.bounds.upper) {
                format!("{}", (l + u) / 2.0)
            } else {
                "1.0".to_string()
            }
        }
        _ => "3.14".to_string(),
    }
}

/// 为 `new()` 方法生成文档
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

/// 生成有效的示例值
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
            Sign::Negative,
            Bounds {
                lower: Some(l),
                upper: Some(u),
            },
        ) => {
            format!("{}", (*l + *u) / 2.0)
        }
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
        _ => "3.14".to_string(),
    }
}

/// 生成无效的示例值（用于展示错误）
pub fn generate_invalid_example(
    float_type: &Ident,
    constraint_def: &ConstraintDef,
) -> (String, &'static str) {
    let _nan_value = if float_type == "f32" {
        "f32::NAN"
    } else {
        "f64::NAN"
    };

    match (
        constraint_def.sign,
        constraint_def.excludes_zero,
        &constraint_def.bounds,
    ) {
        (
            Sign::Positive,
            false,
            Bounds {
                lower: Some(0.0), ..
            },
        ) => ("-1.0".to_string(), "负值"),
        (
            Sign::Positive,
            true,
            Bounds {
                lower: Some(0.0), ..
            },
        ) => ("-1.0".to_string(), "负值"),
        (
            Sign::Negative,
            false,
            Bounds {
                upper: Some(0.0), ..
            },
        ) => ("1.0".to_string(), "正值"),
        (
            Sign::Negative,
            true,
            Bounds {
                upper: Some(0.0), ..
            },
        ) => ("1.0".to_string(), "正值"),
        (
            Sign::Any,
            true,
            Bounds {
                lower: None,
                upper: None,
            },
        ) => ("0.0".to_string(), "零值"),
        (Sign::Any, false, Bounds { lower: Some(l), .. }) if *l > 0.0 => {
            (format!("{}", *l - 1.0), "超出下界")
        }
        (Sign::Any, false, Bounds { upper: Some(u), .. }) if *u < 0.0 => {
            (format!("{}", *u + 1.0), "超出上界")
        }
        _ => (
            format!("{}::NAN", float_type.to_string().to_lowercase()),
            "NaN",
        ),
    }
}
