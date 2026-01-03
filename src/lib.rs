//! # 受约束的浮点数类型
//!
//! 这个模块提供了受约束的浮点数类型，所有类型默认保证 finite（排除 NaN 和无穷大）：
//! - `FinF32` 和 `FinF64`：有限值浮点数（排除 NaN 和无穷大）
//! - `PositiveF32` 和 `PositiveF64`：非负值浮点数（>= 0，有限值）
//! - `NonZeroF32` 和 `NonZeroF64`：非零值浮点数（!= 0，排除 0.0, -0.0, NaN, 无穷大）
//! - `NonZeroPositiveF32` 和 `NonZeroPositiveF64`：非零正值浮点数（> 0，有限值）
//! - `NegativeF32` 和 `NegativeF64`：非正值浮点数（<= 0，有限值）
//! - `NonZeroNegativeF32` 和 `NonZeroNegativeF64`：非零负值浮点数（< 0，有限值）
//! - `NormalizedF32` 和 `NormalizedF64`：标准化浮点数（0.0 <= value <= 1.0，有限值）
//!
//! ## 可组合约束
//!
//! 所有约束都可以自由组合。例如，`NonZeroPositiveF32` 是 `Positive` 和 `NonZero` 的组合：
//!
//! ```
//! use strict_num_extended::*;
//!
//! // 使用预定义的组合类型
//! let nonzero_pos: NonZeroPositiveF32 = NonZeroPositiveF32::new(10.0).unwrap();
//! ```
//!
//! 此外，还提供了对应的 `Option` 版本，用于处理可能失败的运算。
//!
//! # 示例
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
//! # Option 版本
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
//! # 编译期常量
//!
//! ```
//! use strict_num_extended::FinF32;
//!
//! const ONE: FinF32 = FinF32::new_const(1.0);
//! assert_eq!(ONE.get(), 1.0);
//! ```
//!
//! **注意**：`new_const` 方法现在支持编译期验证，会在编译期 panic 如果值不满足约束条件。

// 使用 proc_macro 生成所有代码
strict_num_extended_macros::generate_constrained_types!({
    // 原子约束类型
    constraints: [
        Finite {
            doc: "有限的浮点值（排除 NaN 和无穷大）",
            validate: "!value.is_nan() && !value.is_infinite()"
        },
        Positive {
            doc: "非负的浮点值（>= 0，有限值）",
            validate: "!value.is_nan() && !value.is_infinite() && !value.is_sign_negative()"
        },
        Negative {
            doc: "非正的浮点值（<= 0，有限值）",
            validate: "!(value.is_nan() || value.is_infinite() || (value.is_sign_positive() && value != 0.0))"
        },
        NonZero {
            doc: "非零的浮点值（!= 0.0 && != -0.0）",
            validate: "!value.is_nan() && !value.is_infinite() && value != 0.0 && value != -0.0"
        },
        Normalized {
            doc: "标准化的浮点值（0.0 <= value <= 1.0，有限值）",
            validate: "!value.is_nan() && !value.is_infinite() && value >= 0.0 && value <= 1.0"
        }
    ],

    // 类型定义（统一使用方括号）
    constraint_types: [
        // 单约束类型
        (Fin, [f32, f64], [Finite]),
        (Positive, [f32, f64], [Positive]),
        (Negative, [f32, f64], [Negative]),
        (NonZero, [f32, f64], [NonZero]),
        (Normalized, [f32, f64], [Normalized]),
        // 组合约束类型
        (NonZeroPositive, [f32, f64], [Positive, NonZero]),
        (NonZeroNegative, [f32, f64], [Negative, NonZero]),
    ],

    // 特性配置
    features: {
        impl_traits: [
            PartialEq, Eq, PartialOrd, Ord,
            Display, Debug,
            Add, Sub, Mul, Div
        ],
        generate_option_types: true,
        generate_new_const: true
    }
});
