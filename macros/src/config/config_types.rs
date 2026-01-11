//! Type definitions for configuration structures
//!
//! Contains all type definitions used in configuration parsing and arithmetic inference.

use proc_macro2::Ident;
use std::collections::HashMap;

// ============================================================================
// Sign and bound type definitions for arithmetic operations
// ============================================================================

/// Sign property of a constraint type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    /// Non-negative values (>= 0)
    Positive,
    /// Non-positive values (<= 0)
    Negative,
    /// Any sign (including zero)
    Any,
}

/// Bound information for a constraint type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Lower bound (None means -∞)
    pub lower: Option<f64>,
    /// Upper bound (None means +∞)
    pub upper: Option<f64>,
}

impl Bounds {
    /// Check if this type is bounded (has both upper and lower bounds)
    pub const fn is_bounded(&self) -> bool {
        self.lower.is_some() && self.upper.is_some()
    }
}

/// Arithmetic operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Result of arithmetic operation type inference.
#[derive(Debug, Clone)]
pub struct ArithmeticResult {
    /// Output constraint type name
    pub output_type: Ident,
    /// Whether this operation is safe (no overflow/divide-by-zero possible)
    pub is_safe: bool,
}

// ============================================================================
// Configuration structure definitions
// ============================================================================

/// Main configuration structure.
pub struct TypeConfig {
    /// List of constraint definitions.
    pub constraints: Vec<ConstraintDef>,
    /// List of constraint type definitions.
    pub constraint_types: Vec<TypeDef>,
    /// Arithmetic operation results: (op, `lhs_name`, `rhs_name`) -> `ArithmeticResult`
    pub arithmetic_results: HashMap<(ArithmeticOp, String, String), ArithmeticResult>,
}

/// Single constraint definition.
pub struct ConstraintDef {
    /// Constraint name.
    pub name: Ident,
    /// Name of the constraint type after negation (e.g., Positive -> Negative).
    pub neg_constraint_name: Option<Ident>,
    /// Raw conditions before adding `is_finite()` check (used for negation calculation).
    pub raw_conditions: Vec<String>,
    /// Sign property of this constraint.
    pub sign: Sign,
    /// Bound information of this constraint.
    pub bounds: Bounds,
    /// Whether this constraint excludes zero.
    pub excludes_zero: bool,
}

/// Type definition (single constraint).
pub struct TypeDef {
    /// Type name.
    pub type_name: Ident,
    /// List of floating-point types.
    pub float_types: Vec<Ident>,
    /// Constraint name.
    pub constraint_name: Ident,
}

/// Gets standard arithmetic operator definition array
///
/// Contains four basic arithmetic operations (addition, subtraction, multiplication, division)
/// and their corresponding:
/// - Operator enum
/// - Trait name (e.g., "Add")
/// - Method name (e.g., "add")
/// - Operator symbol (e.g., quote! { + })
///
/// # Returns
///
/// An array containing all standard arithmetic operation definitions
pub fn get_standard_arithmetic_ops() -> [(
    ArithmeticOp,
    &'static str,
    &'static str,
    proc_macro2::TokenStream,
); 4] {
    use quote::quote;

    [
        (ArithmeticOp::Add, "Add", "add", quote! { + }),
        (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
        (ArithmeticOp::Mul, "Mul", "mul", quote! { * }),
        (ArithmeticOp::Div, "Div", "div", quote! { / }),
    ]
}
