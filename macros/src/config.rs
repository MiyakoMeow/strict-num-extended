//! Configuration structure and parsing module
//!
//! Defines configuration structures for procedural macros and `TokenStream` parsing logic.

use proc_macro2::Ident;
use syn::{Expr, parse::Parse, parse::ParseStream};

// ============================================================================
// Configuration structure definitions
// ============================================================================

/// Main configuration structure.
pub struct TypeConfig {
    /// List of constraint definitions.
    pub constraints: Vec<ConstraintDef>,
    /// List of constraint type definitions.
    pub constraint_types: Vec<TypeDef>,
    /// Feature configuration.
    pub features: FeaturesConfig,
}

/// Single constraint definition.
pub struct ConstraintDef {
    /// Constraint name.
    pub name: Ident,
    /// Constraint documentation.
    pub doc: String,
    /// Validation expression.
    pub validate: String,
}

/// Type definition (single or combined constraint).
pub enum TypeDef {
    /// Single constraint type.
    Single {
        /// Type name.
        type_name: Ident,
        /// List of floating-point types.
        float_types: Vec<Ident>,
        /// Constraint name.
        constraint_name: Ident,
    },
    /// Combined constraint type.
    Combined {
        /// Type name.
        type_name: Ident,
        /// List of floating-point types.
        float_types: Vec<Ident>,
        /// List of constraint names.
        constraints: Vec<Ident>,
    },
}

impl TypeDef {
    /// Get the type name (applies to both Single and Combined)
    pub const fn type_name(&self) -> &Ident {
        match self {
            TypeDef::Single { type_name, .. } | TypeDef::Combined { type_name, .. } => type_name,
        }
    }

    /// Get the list of floating-point types (applies to both Single and Combined)
    pub fn float_types(&self) -> &[Ident] {
        match self {
            TypeDef::Single { float_types, .. } => float_types,
            TypeDef::Combined { float_types, .. } => float_types,
        }
    }
}

/// Feature configuration.
pub struct FeaturesConfig {
    /// List of traits to implement.
    pub impl_traits: Vec<Ident>,
    /// Whether to generate Option types.
    pub generate_option_types: bool,
    /// Whether to generate const new methods.
    pub generate_new_const: bool,
}

// ============================================================================
// Parse trait implementations
// ============================================================================

impl Parse for TypeConfig {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse braced content
        let content;
        syn::braced!(content in input);

        let mut constraints = Vec::new();
        let mut constraint_types = Vec::new();
        let mut impl_traits = Vec::new();
        let mut generate_option_types = true;
        let mut generate_new_const = true;

        // Parse each field
        while !content.is_empty() {
            let ident: Ident = content.parse()?;

            match ident.to_string().as_str() {
                "constraints" => {
                    content.parse::<syn::Token![:]>()?;
                    constraints = parse_constraints(&content)?;
                }
                "constraint_types" => {
                    content.parse::<syn::Token![:]>()?;
                    constraint_types = parse_constraint_types(&content)?;
                }
                "features" => {
                    content.parse::<syn::Token![:]>()?;
                    let feature_content;
                    syn::braced!(feature_content in content);

                    while !feature_content.is_empty() {
                        let feature_ident: Ident = feature_content.parse()?;
                        feature_content.parse::<syn::Token![:]>()?;

                        match feature_ident.to_string().as_str() {
                            "impl_traits" => {
                                impl_traits = parse_ident_list(&feature_content)?;
                            }
                            "generate_option_types" => {
                                generate_option_types = parse_bool_lit(&feature_content)?;
                            }
                            "generate_new_const" => {
                                generate_new_const = parse_bool_lit(&feature_content)?;
                            }
                            _ => {
                                return Err(syn::Error::new(
                                    feature_ident.span(),
                                    format!("Unknown feature: {}", feature_ident),
                                ));
                            }
                        }

                        let _ = feature_content.parse::<syn::Token![,]>();
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown field: {}", ident),
                    ));
                }
            }

            let _ = content.parse::<syn::Token![,]>();
        }

        Ok(TypeConfig {
            constraints,
            constraint_types,
            features: FeaturesConfig {
                impl_traits,
                generate_option_types,
                generate_new_const,
            },
        })
    }
}

/// Parse constraint definition list.
pub fn parse_constraints(input: ParseStream) -> syn::Result<Vec<ConstraintDef>> {
    let mut constraints = Vec::new();
    let content;
    syn::bracketed!(content in input);

    while !content.is_empty() {
        let name: Ident = content.parse()?;
        let field_content;
        syn::braced!(field_content in content);

        let mut doc = String::new();
        let mut validate = None;

        while !field_content.is_empty() {
            let ident: Ident = field_content.parse()?;
            field_content.parse::<syn::Token![:]>()?;

            match ident.to_string().as_str() {
                "doc" => {
                    if let Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = field_content.parse()?
                    {
                        doc = s.value();
                    }
                }
                "validate" => {
                    if let Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(s),
                        ..
                    }) = field_content.parse()?
                    {
                        validate = Some(s.value());
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("Unknown field in constraint: {}", ident),
                    ));
                }
            }

            let _ = field_content.parse::<syn::Token![,]>();
        }

        let validate = validate
            .ok_or_else(|| syn::Error::new(name.span(), "Missing validate field in constraint"))?;

        constraints.push(ConstraintDef {
            name,
            doc,
            validate,
        });

        let _ = content.parse::<syn::Token![,]>();
    }

    Ok(constraints)
}

/// Parse constraint type definitions (uniformly use square brackets).
pub fn parse_constraint_types(input: ParseStream) -> syn::Result<Vec<TypeDef>> {
    let mut types = Vec::new();
    let content;
    syn::bracketed!(content in input);

    while !content.is_empty() {
        // Parse (Name, [f32, f64], [Constraint1, Constraint2, ...])
        let paren_content;
        syn::parenthesized!(paren_content in content);

        let type_name: Ident = paren_content.parse()?;
        paren_content.parse::<syn::Token![,]>()?;

        let float_types = parse_ident_list(&paren_content)?;
        paren_content.parse::<syn::Token![,]>()?;

        // Always parse as a list (single constraints are also lists)
        let constraints = parse_ident_list(&paren_content)?;

        // Decide Single or Combined based on constraint count
        if constraints.len() == 1 {
            // Single constraint type
            let constraint_name = constraints
                .first()
                .expect("constraints should have at least one element");
            types.push(TypeDef::Single {
                type_name,
                float_types,
                constraint_name: constraint_name.clone(),
            });
        } else {
            // Combined constraint type
            types.push(TypeDef::Combined {
                type_name,
                float_types,
                constraints,
            });
        }

        let _ = content.parse::<syn::Token![,]>();
    }

    Ok(types)
}

/// Parse identifier list.
pub fn parse_ident_list(input: ParseStream) -> syn::Result<Vec<Ident>> {
    let mut idents = Vec::new();
    let content;
    syn::bracketed!(content in input);

    while !content.is_empty() {
        let ident: Ident = content.parse()?;
        idents.push(ident);
        let _ = content.parse::<syn::Token![,]>();
    }

    Ok(idents)
}

/// Parse `LitBool` value from `ParseStream`.
fn parse_bool_lit(input: ParseStream) -> syn::Result<bool> {
    let lit: syn::LitBool = input.parse()?;
    Ok(lit.value)
}
