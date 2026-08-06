use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn uint_addition() {
    let evaluator = eval_program("dec uint x = (10 as uint) + (5 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(15)));
}

#[test]
fn uint_subtraction() {
    let evaluator = eval_program("dec uint x = (10 as uint) - (5 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(5)));
}

#[test]
fn uint_multiplication() {
    let evaluator = eval_program("dec uint x = (10 as uint) * (5 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(50)));
}

#[test]
fn uint_division() {
    let evaluator = eval_program("dec uint x = (10 as uint) / (5 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(2)));
}

#[test]
fn small_uint_addition() {
    let evaluator =
        eval_program("dec small uint x = (10 as small uint) + (5 as small uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(15)));
}

#[test]
fn small_uint_subtraction() {
    let evaluator =
        eval_program("dec small uint x = (10 as small uint) - (5 as small uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(5)));
}

#[test]
fn small_uint_multiplication() {
    let evaluator =
        eval_program("dec small uint x = (10 as small uint) * (5 as small uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(50)));
}

#[test]
fn small_uint_division() {
    let evaluator =
        eval_program("dec small uint x = (10 as small uint) / (5 as small uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(2)));
}

#[test]
fn uint_add_overflow_is_error() {
    assert!(eval_program("dec uint x = (18446744073709551615 as uint) + (1 as uint)").is_err());
}

#[test]
fn uint_sub_underflow_is_error() {
    assert!(eval_program("dec uint x = (1 as uint) - (2 as uint)").is_err());
}

#[test]
fn uint_mul_overflow_is_error() {
    assert!(eval_program("dec uint x = (18446744073709551615 as uint) * (2 as uint)").is_err());
}

#[test]
fn uint_div_by_zero_is_error() {
    assert!(eval_program("dec uint x = (1 as uint) / (0 as uint)").is_err());
}

#[test]
fn small_uint_add_overflow_is_error() {
    assert!(
        eval_program("dec small uint x = (4294967295 as small uint) + (1 as small uint)").is_err()
    );
}

#[test]
fn uint_mixed_with_int_is_error() {
    assert!(eval_program("dec uint x = (1 as uint) + 2").is_err());
}

#[test]
fn uint_mixed_with_small_uint_is_error() {
    assert!(eval_program("dec uint x = (1 as uint) + (2 as small uint)").is_err());
}

#[test]
fn uint_less_than() {
    let evaluator = eval_program("dec bool x = (1 as uint) < (2 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn uint_greater_equal() {
    let evaluator = eval_program("dec bool x = (3 as uint) >= (2 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn uint_not_equal() {
    let evaluator = eval_program("dec bool x = (3 as uint) != (2 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Bool(true)));
}

#[test]
fn uint_equal() {
    let evaluator = eval_program("dec bool x = (3 as uint) == (3 as uint)").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Bool(true)));
}
