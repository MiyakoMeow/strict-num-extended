//! Arithmetic operations module

mod binary_ops;
mod neg_ops;

// Re-export all functions
pub use binary_ops::generate_arithmetic_impls;
pub use neg_ops::generate_neg_impls;
