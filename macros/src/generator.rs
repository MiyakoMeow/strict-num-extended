//! Code generation module
//!
//! Contains helper functions and re-exports all code generation functionality.

mod iterators;
mod type_utils;
mod validation;

// Re-export all functions
pub use iterators::{generate_arithmetic_for_all_types, generate_arithmetic_for_primitive_types};
pub use type_utils::{
    filter_constraint_types_by_float, find_constraint_def, for_all_constraint_float_types,
    make_type_alias,
};
pub use validation::build_validation_expr;
