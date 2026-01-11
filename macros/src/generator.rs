//! Code generation module
//!
//! Contains helper functions and re-exports all code generation functionality.

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};

use crate::config::{ArithmeticOp, ArithmeticResult, ConstraintDef, TypeConfig};

// ============================================================================
// Helper functions (shared by multiple modules)
// ============================================================================

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

/// Generates arithmetic operation implementations for all constraint type combinations
///
/// This function encapsulates the common logic of iterating through lhs × rhs × `float_types`
/// combinations and looking up result types from the precomputed arithmetic results table.
///
/// # Arguments
///
/// * `config` - Type configuration
/// * `ops` - Operator definition array, format: (operator, trait name, method name, operator symbol)
/// * `impl_generator` - User-provided implementation generator function
///
/// # Returns
///
/// `TokenStream` of all generated arithmetic operation implementations
///
/// # Examples
///
/// ```ignore
/// let ops = [
///     (ArithmeticOp::Add, "Add", "add", quote! { + }),
///     (ArithmeticOp::Sub, "Sub", "sub", quote! { - }),
/// ];
///
/// generate_arithmetic_for_all_types(config, &ops, |lhs, rhs, output, trait_ident, method_ident, op_symbol, result, op| {
///     // Generate specific trait implementation
///     quote! {
///         impl #trait_ident for #lhs {
///             // ...
///         }
///     }
/// })
/// ```
pub fn generate_arithmetic_for_all_types<F>(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
    mut impl_generator: F,
) -> TokenStream2
where
    F: FnMut(
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        TokenStream2,
        &ArithmeticResult,
        ArithmeticOp,
        bool,
    ) -> TokenStream2,
{
    let mut impls = Vec::new();

    for lhs_type in &config.constraint_types {
        for rhs_type in &config.constraint_types {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Get arithmetic result from precomputed table
                let key = (
                    *op,
                    lhs_type.type_name.to_string(),
                    rhs_type.type_name.to_string(),
                );
                let result = config
                    .arithmetic_results
                    .get(&key)
                    .expect("Arithmetic result not found");

                for float_type in &lhs_type.float_types {
                    let lhs_alias = make_type_alias(&lhs_type.type_name, float_type);
                    let rhs_alias = make_type_alias(&rhs_type.type_name, float_type);
                    let output_alias = make_type_alias(&result.output_type, float_type);

                    let impl_code = impl_generator(
                        lhs_alias,
                        rhs_alias,
                        output_alias,
                        trait_ident.clone(),
                        method_ident.clone(),
                        op_symbol.clone(),
                        result,
                        *op,
                        false, // not reversed for constraint-constraint operations
                    );

                    impls.push(impl_code);
                }
            }
        }
    }

    quote! {
        #(#impls)*
    }
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
pub fn for_all_constraint_float_types<F>(config: &TypeConfig, mut generator: F) -> Vec<TokenStream2>
where
    F: FnMut(&Ident, &Ident, &ConstraintDef) -> TokenStream2,
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

/// Dynamically builds validation expression based on constraint definition
pub fn build_validation_expr(constraint_def: &ConstraintDef, float_type: &Ident) -> TokenStream2 {
    let mut checks = Vec::new();

    // 1. Base check: is_finite()
    checks.push(quote! { value.is_finite() });

    // 2. Boundary checks
    if let Some(lower) = constraint_def.bounds.lower {
        let lower_check = build_bound_check(lower, true, constraint_def.excludes_zero, float_type);
        checks.push(lower_check);
    }

    if let Some(upper) = constraint_def.bounds.upper {
        let upper_check = build_bound_check(upper, false, constraint_def.excludes_zero, float_type);
        checks.push(upper_check);
    }

    // 3. Zero exclusion check (if not covered by bounds)
    if constraint_def.excludes_zero && needs_explicit_zero_check(constraint_def) {
        checks.push(quote! { value != 0.0 });
    }

    // Combine all checks with &&
    quote! {
        #(#checks)&&*
    }
}

/// Builds a single boundary check expression
fn build_bound_check(
    bound: f64,
    is_lower: bool,
    excludes_zero: bool,
    float_type: &Ident,
) -> TokenStream2 {
    let is_f32 = *float_type == "f32";

    // For strict inequalities with excludes_zero and bound at zero, use strict comparison
    // Otherwise use non-strict comparison
    let use_strict = excludes_zero && bound == 0.0;

    // Determine the bound value to use (no substitution for strict inequalities)
    let bound_value = if is_f32 {
        quote! { (#bound as f64) }
    } else {
        quote! { #bound }
    };

    // f32 needs to convert value to f64 for comparison
    let value_expr = if is_f32 {
        quote! { (value as f64) }
    } else {
        quote! { value }
    };

    // Generate the appropriate comparison expression
    if is_lower {
        if use_strict {
            // For > x with excludes_zero and x == 0, use > to exclude 0.0 and -0.0
            quote! { #value_expr > #bound_value }
        } else {
            quote! { #value_expr >= #bound_value }
        }
    } else if use_strict {
        // For < x with excludes_zero and x == 0, use < to exclude 0.0 and -0.0
        quote! { #value_expr < #bound_value }
    } else {
        quote! { #value_expr <= #bound_value }
    }
}

/// Checks if an explicit zero check is needed
fn needs_explicit_zero_check(constraint_def: &ConstraintDef) -> bool {
    // If bounds already exclude zero through strict comparison (> or <), no need for explicit check
    let lower_excludes_zero =
        constraint_def.bounds.lower == Some(0.0) && constraint_def.excludes_zero;
    let upper_excludes_zero =
        constraint_def.bounds.upper == Some(0.0) && constraint_def.excludes_zero;

    // Need explicit check if bounds don't cover zero exclusion
    !lower_excludes_zero && !upper_excludes_zero
}

/// Generates arithmetic operation implementations for constraint types with primitive types (f32, f64).
///
/// This function generates implementations for:
/// 1. Constraint type op primitive type (e.g., `FinF64` + f64)
/// 2. Primitive type op constraint type (e.g., f64 + `FinF64`)
///
/// The primitive type is treated as a Fin constraint, and the result type is determined
/// by the existing arithmetic results table.
///
/// # Arguments
///
/// * `config` - Type configuration
/// * `ops` - Operator definition array
/// * `impl_generator` - User-provided implementation generator function
///
/// # Returns
///
/// `TokenStream` of all generated arithmetic operation implementations
pub fn generate_arithmetic_for_primitive_types<F>(
    config: &TypeConfig,
    ops: &[(ArithmeticOp, &str, &str, TokenStream2)],
    mut impl_generator: F,
) -> TokenStream2
where
    F: FnMut(
        Ident,
        Ident,
        Ident,
        Ident,
        Ident,
        TokenStream2,
        &ArithmeticResult,
        ArithmeticOp,
        bool,
    ) -> TokenStream2,
{
    let mut impls = Vec::new();

    // Define primitive type mappings - use "Fin" as the constraint name
    let primitive_mappings = vec![("f32", "Fin"), ("f64", "Fin")];

    // 1. Constraint type op primitive type (e.g., FinF64 + f64)
    for lhs_type in &config.constraint_types {
        for (primitive_name, fin_constraint) in &primitive_mappings {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Look up the arithmetic result (constraint type op Fin constraint)
                let key = (
                    *op,
                    lhs_type.type_name.to_string(),
                    fin_constraint.to_string(),
                );
                if let Some(result) = config.arithmetic_results.get(&key) {
                    for float_type in &lhs_type.float_types {
                        // Only generate operations where the primitive type matches the float type
                        if float_type.to_string().as_str() != *primitive_name {
                            continue;
                        }
                        let lhs_alias = make_type_alias(&lhs_type.type_name, float_type);
                        let primitive_ident = Ident::new(primitive_name, Span::call_site());
                        let output_alias = make_type_alias(&result.output_type, float_type);

                        let impl_code = impl_generator(
                            lhs_alias,
                            primitive_ident,
                            output_alias,
                            trait_ident.clone(),
                            method_ident.clone(),
                            op_symbol.clone(),
                            result,
                            *op,
                            false, // not reversed
                        );

                        impls.push(impl_code);
                    }
                }
            }
        }
    }

    // 2. Primitive type op constraint type (e.g., f64 + FinF64)
    for (primitive_name, fin_constraint) in &primitive_mappings {
        for rhs_type in &config.constraint_types {
            for (op, trait_name, method_name, op_symbol) in ops {
                let trait_ident = Ident::new(trait_name, Span::call_site());
                let method_ident = Ident::new(method_name, Span::call_site());

                // Look up the arithmetic result (Fin constraint op constraint type)
                let key = (
                    *op,
                    fin_constraint.to_string(),
                    rhs_type.type_name.to_string(),
                );
                if let Some(result) = config.arithmetic_results.get(&key) {
                    for float_type in &rhs_type.float_types {
                        // Only generate operations where the primitive type matches the float type
                        if float_type.to_string().as_str() != *primitive_name {
                            continue;
                        }
                        let primitive_ident = Ident::new(primitive_name, Span::call_site());
                        let rhs_alias = make_type_alias(&rhs_type.type_name, float_type);
                        let output_alias = make_type_alias(&result.output_type, float_type);

                        let impl_code = impl_generator(
                            primitive_ident,
                            rhs_alias,
                            output_alias,
                            trait_ident.clone(),
                            method_ident.clone(),
                            op_symbol.clone(),
                            result,
                            *op,
                            true, // reversed: primitive is on the left
                        );

                        impls.push(impl_code);
                    }
                }
            }
        }
    }

    quote! {
        #(#impls)*
    }
}
