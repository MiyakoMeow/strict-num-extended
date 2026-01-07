//! Result arithmetic operations tests
//!
//! Tests for type-safe arithmetic operations with Result<T, FloatError> types.

// Strict floating-point comparisons, unwrap usage, and variable shadowing in test code are justified
#![allow(clippy::unwrap_used, clippy::shadow_unrelated)]

use strict_num_extended::*;

/// Unified macro for testing Result arithmetic operations
///
/// Supports multiple calling formats for Result types:
/// - Both sides Ok: `test_result_arith!(name, Ok(a), Ok(b), op, Result, Ok(expected))`
/// - LHS Err: `test_result_arith!(name, Err, Ok(b), op, Result, Err)`
/// - RHS Err: `test_result_arith!(name, Ok(a), Err, op, Result, Err)`
/// - Concrete LHS: `test_result_arith!(name, a, Ok(b), op, Result, Ok(expected))`
/// - Concrete RHS: `test_result_arith!(name, Ok(a), b, op, Result, Ok(expected))`
#[macro_export]
macro_rules! test_result_arith {
    // Result<LHS> op Result<RHS> with Ok result
    ($test_name:ident, Ok($a:expr), Ok($b:expr), $op:tt, Result<$ResultType:ty, FloatError>, Ok($expected:expr)) => {
        #[test]
        fn $test_name() {
            let a: Result<$crate::PositiveF64, FloatError> = Ok($crate::PositiveF64::new_const($a));
            let b: Result<$crate::NegativeF64, FloatError> = Ok($crate::NegativeF64::new_const($b));
            let result: Result<$ResultType, FloatError> = a $op b;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().get(), $expected);
        }
    };
    // Result<LHS> op Result<RHS> with Err result
    ($test_name:ident, Err, Ok($b:expr), $op:tt, Result<$ResultType:ty, FloatError>, Err) => {
        #[test]
        fn $test_name() {
            let a: Result<$crate::PositiveF64, FloatError> = Err(FloatError::NaN);
            let b: Result<$crate::NegativeF64, FloatError> = Ok($crate::NegativeF64::new_const($b));
            let result: Result<$ResultType, FloatError> = a $op b;
            assert!(result.is_err());
        }
    };
    // Concrete LHS op Result<RHS> with Ok result
    ($test_name:ident, $TypeA:ty, $a:expr, Ok($b:expr), $op:tt, Result<$ResultType:ty, FloatError>, Ok($expected:expr)) => {
        #[test]
        fn $test_name() {
            const A: $TypeA = <$TypeA>::new_const($a);
            let b: Result<$crate::NegativeF64, FloatError> = Ok($crate::NegativeF64::new_const($b));
            let result: Result<$ResultType, FloatError> = A $op b;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().get(), $expected);
        }
    };
    // Result<LHS> op Concrete RHS with Ok result
    ($test_name:ident, Ok($a:expr), $TypeB:ty, $b:expr, $op:tt, Result<$ResultType:ty, FloatError>, Ok($expected:expr)) => {
        #[test]
        fn $test_name() {
            let a: Result<$crate::PositiveF64, FloatError> = Ok($crate::PositiveF64::new_const($a));
            const B: $TypeB = <$TypeB>::new_const($b);
            let result: Result<$ResultType, FloatError> = a $op B;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().get(), $expected);
        }
    };
}

mod test_result_lhs_concrete_rhs {
    use super::*;

    #[test]
    fn test_ok_positive_plus_negative() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(5.0));
        const B: NegativeF64 = NegativeF64::new_const(-3.0);
        let result: Result<FinF64, FloatError> = a + B;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 2.0);
    }

    #[test]
    fn test_err_positive_plus_negative() {
        let a: Result<PositiveF64, FloatError> = Err(FloatError::NaN);
        const B: NegativeF64 = NegativeF64::new_const(-3.0);
        let result: Result<FinF64, FloatError> = a + B;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::NaN);
    }

    #[test]
    fn test_ok_positive_mul_negative() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(5.0));
        const B: NegativeF64 = NegativeF64::new_const(-3.0);
        let result: Result<NegativeF64, FloatError> = a * B;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), -15.0);
    }
}

mod test_concrete_lhs_result_rhs {
    use super::*;

    #[test]
    fn test_positive_plus_ok_negative() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Result<NegativeF64, FloatError> = Ok(NegativeF64::new_const(-3.0));
        let result: Result<FinF64, FloatError> = A + b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 2.0);
    }

    #[test]
    fn test_positive_plus_err_negative() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Result<NegativeF64, FloatError> = Err(FloatError::NaN);
        let result: Result<FinF64, FloatError> = A + b;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::NaN);
    }

    #[test]
    fn test_positive_mul_ok_negative() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Result<NegativeF64, FloatError> = Ok(NegativeF64::new_const(-3.0));
        let result: Result<NegativeF64, FloatError> = A * b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), -15.0);
    }
}

mod test_result_both_sides {
    use super::*;

    // Note: Pattern 3 (Result op Result) violates orphan rule and is not implemented.
    // Users can use .and_then() or pattern matching instead:
    // let result = a.and_then(|a_val| b.map(|b_val| a_val + b_val));
}

mod test_result_negation {
    use super::*;

    // Note: Result negation violates orphan rule and is not implemented.
    // Users can use .map() instead:
    // let result = a.map(|x| -x);
}

mod test_result_error_propagation {
    use super::*;

    #[test]
    fn test_nan_propagation() {
        let a: Result<PositiveF64, FloatError> = Err(FloatError::NaN);
        const B: NegativeF64 = NegativeF64::new_const(-3.0);
        let result: Result<FinF64, FloatError> = a + B;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::NaN);
    }

    #[test]
    fn test_pos_inf_propagation() {
        const A: PositiveF64 = PositiveF64::new_const(5.0);
        let b: Result<NegativeF64, FloatError> = Err(FloatError::PosInf);
        let result: Result<FinF64, FloatError> = A + b;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::PosInf);
    }

    #[test]
    fn test_out_of_range_propagation() {
        let a: Result<PositiveF64, FloatError> = Err(FloatError::OutOfRange);
        let b: NegativeF64 = NegativeF64::new_const(-3.0);
        let result: Result<FinF64, FloatError> = a + b;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::OutOfRange);
    }
}

mod test_result_division_edge_cases {
    use super::*;

    #[test]
    fn test_division_by_zero_ok_ok() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(10.0));
        let b: PositiveF64 = PositiveF64::new_const(0.0);
        let result: Result<PositiveF64, FloatError> = a / b;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::DivisionByZero);
    }

    #[test]
    fn test_division_by_zero_ok_concrete() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(10.0));
        const B: PositiveF64 = PositiveF64::new_const(0.0);
        let result: Result<PositiveF64, FloatError> = a / B;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::DivisionByZero);
    }

    #[test]
    fn test_division_by_zero_concrete_ok() {
        const A: PositiveF64 = PositiveF64::new_const(10.0);
        let b: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(0.0));
        let result: Result<PositiveF64, FloatError> = A / b;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FloatError::DivisionByZero);
    }

    #[test]
    fn test_safe_division() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(10.0));
        let b: PositiveF64 = PositiveF64::new_const(2.0);
        let result: Result<PositiveF64, FloatError> = a / b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 5.0);
    }
}

mod test_result_chaining {
    use super::*;

    #[test]
    fn test_chain_operations() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(10.0));
        let b: NegativeF64 = NegativeF64::new_const(-3.0);
        let c: PositiveF64 = PositiveF64::new_const(2.0);

        // (a + b) * c = (10 - 3) * 2 = 14
        let step1 = a + b;
        assert!(step1.is_ok());
        let result = step1.and_then(|val| val * c);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 14.0);
    }

    #[test]
    fn test_chain_with_error() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(10.0));
        let b: NegativeF64 = NegativeF64::new_const(-3.0);

        // (a + b) should succeed
        let step1: Result<FinF64, FloatError> = a + b;
        assert!(step1.is_ok());
        assert_eq!(step1.unwrap().get(), 7.0);

        // Now test with actual error - cannot use Result op Result, use concrete type
        let a2: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(10.0));
        let b2: NegativeF64 = NegativeF64::new_const(-3.0);
        let step2: Result<FinF64, FloatError> = a2 + b2;
        assert!(step2.is_ok());
        assert_eq!(step2.unwrap().get(), 7.0);
    }

    #[test]
    fn test_complex_chain() {
        let a: Result<PositiveF64, FloatError> = Ok(PositiveF64::new_const(5.0));
        let b: PositiveF64 = PositiveF64::new_const(3.0);
        let c: NegativeF64 = NegativeF64::new_const(-2.0);

        // ((a * b) + c) = ((5 * 3) + (-2)) = 13
        let step1 = a * b;
        assert!(step1.is_ok());
        let result = step1.and_then(|val| Ok(val + c));
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 13.0);
    }
}

mod test_result_cross_type_operations {
    use super::*;

    #[test]
    fn test_normalized_mul_normalized() {
        let a: Result<NormalizedF64, FloatError> = Ok(NormalizedF64::new_const(0.5));
        let b: NormalizedF64 = NormalizedF64::new_const(0.5);
        let result: Result<NormalizedF64, FloatError> = a * b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0.25);
    }

    #[test]
    fn test_symmetric_add_symmetric() {
        let a: Result<SymmetricF64, FloatError> = Ok(SymmetricF64::new_const(0.5));
        let b: SymmetricF64 = SymmetricF64::new_const(-0.3);
        let result: Result<FinF64, FloatError> = a + b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 0.2);
    }

    #[test]
    fn test_nonzero_positive_add_nonzero_negative() {
        let a: Result<NonZeroPositiveF64, FloatError> = Ok(NonZeroPositiveF64::new_const(5.0));
        let b: NonZeroNegativeF64 = NonZeroNegativeF64::new_const(-3.0);
        let result: Result<FinF64, FloatError> = a + b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().get(), 2.0);
    }
}
