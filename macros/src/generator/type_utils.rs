//! Type utility functions for code generation
//!
//! Contains helper functions for type manipulation and lookup.

use proc_macro2::Ident;
use quote::format_ident;

use crate::config::{ConstraintDef, TypeConfig};

/// Generates type alias identifier from type name and floating-point type
///
/// # Examples
///
/// - `make_type_alias("Positive", "f32")` → `PositiveF32`
/// - `make_type_alias("Negative", "f64")` → `NegativeF64`
///
/// # Arguments
///
/// * `type_name` - Type name (e.g., `Positive`, `Negative`)
/// * `float_type` - Floating-point type (e.g., `f32`, `f64`)
///
/// # Returns
///
/// The combined type alias identifier
pub fn make_type_alias(type_name: &Ident, float_type: &Ident) -> Ident {
    format_ident!("{}{}", type_name, float_type.to_string().to_uppercase())
}

/// Finds constraint definition by constraint name
///
/// # Arguments
///
/// * `config` - Type configuration
/// * `constraint_name` - Constraint name (e.g., `Positive`, `Negative`)
///
/// # Returns
///
/// Reference to the found constraint definition
///
/// # Panics
///
/// Panics if the corresponding constraint definition is not found
pub fn find_constraint_def<'a>(
    config: &'a TypeConfig,
    constraint_name: &Ident,
) -> &'a ConstraintDef {
    config
        .constraints
        .iter()
        .find(|c| &c.name == constraint_name)
        .expect("Constraint not found")
}

/// Filters constraint types that include the specified floating-point type
///
/// # Arguments
///
/// * `config` - Type configuration
/// * `float_type` - Floating-point type identifier (e.g., `f32`, `f64`)
///
/// # Returns
///
/// Collection of all constraint types that include this floating-point type
///
/// # Examples
///
/// ```ignore
/// let f32_types = filter_constraint_types_by_float(config, &format_ident!("f32"));
/// ```
pub fn filter_constraint_types_by_float<'a>(
    config: &'a TypeConfig,
    float_type: &Ident,
) -> Vec<&'a crate::config::TypeDef> {
    config
        .constraint_types
        .iter()
        .filter(|tt| tt.float_types.contains(float_type))
        .collect()
}

/// Iterate over all constraint types and float types, generating code for each combination.
///
/// This function encapsulates the common pattern of iterating through all constraint types
/// and their associated float types, providing the constraint definition and type names
/// to a generator function.
///
/// # Arguments
///
/// * `config` - Type configuration containing constraint definitions
/// * `generator` - Function that generates code for each (`type_name`, `float_type`, `constraint_def`) combination
///
/// # Returns
///
/// A vector of generated token streams
pub fn for_all_constraint_float_types<F>(
    config: &TypeConfig,
    mut generator: F,
) -> Vec<proc_macro2::TokenStream>
where
    F: FnMut(&Ident, &Ident, &ConstraintDef) -> proc_macro2::TokenStream,
{
    let mut results = Vec::new();

    for type_def in &config.constraint_types {
        let type_name = &type_def.type_name;
        let constraint_def = config
            .constraints
            .iter()
            .find(|c| c.name == type_def.constraint_name)
            .expect("Constraint not found");

        for float_type in &type_def.float_types {
            results.push(generator(type_name, float_type, constraint_def));
        }
    }

    results
}
