//! Serde serialization/deserialization tests
//!
//! These tests only run when the "serde" feature is enabled.

#![cfg(feature = "serde")]

use strict_num_extended::*;

#[test]
fn test_serialize_deserialize_fin_f32() {
    let original = FinF32::new(std::f32::consts::PI).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: FinF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_positive_f64() {
    let original = PositiveF64::new(42.0).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PositiveF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_deserialize_normalized_f32() {
    let original = NormalizedF32::new(0.75).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: NormalizedF32 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}

#[test]
fn test_serialize_error() {
    let error = FloatError::NaN;
    let json = serde_json::to_string(&error).unwrap();
    let deserialized: FloatError = serde_json::from_str(&json).unwrap();
    assert_eq!(error, deserialized);
}

#[test]
fn test_serialize_deserialize_symmetric_f64() {
    let original = SymmetricF64::new(0.5).unwrap();
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: SymmetricF64 = serde_json::from_str(&json).unwrap();
    assert_eq!(original.get(), deserialized.get());
}
