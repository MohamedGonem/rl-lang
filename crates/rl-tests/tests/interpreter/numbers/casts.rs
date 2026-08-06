use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn int_to_uint() {
    let evaluator = eval_program("dec uint x = 42 as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(42)));
}

#[test]
fn float_to_uint_truncates() {
    let evaluator = eval_program("dec uint x = 1.5 as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(1)));
}

#[test]
fn small_uint_to_uint() {
    let evaluator = eval_program("dec uint x = (42 as small uint) as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(42)));
}

#[test]
fn uint_to_int() {
    let evaluator = eval_program("dec int x = (42 as uint) as int").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn uint_to_float() {
    let evaluator = eval_program("dec float x = (42 as uint) as float").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(42.0)));
}

#[test]
fn uint_to_small_uint() {
    let evaluator = eval_program("dec small uint x = (42 as uint) as small uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(42)));
}

#[test]
fn uint_to_byte() {
    let evaluator = eval_program("dec byte x = (42 as uint) as byte").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Byte(42)));
}

#[test]
fn uint_to_small_uint_overflow_is_error() {
    assert!(eval_program("dec small uint x = (4294967296 as uint) as small uint").is_err());
}

#[test]
fn int_to_uint_negative_is_error() {
    assert!(eval_program("dec uint x = (-1 as int) as uint").is_err());
}

#[test]
fn float_to_uint_negative_is_error() {
    assert!(eval_program("dec uint x = (-1.0 as float) as uint").is_err());
}

#[test]
fn uint_small_uint_mismatch_is_error() {
    assert!(eval_program("dec uint x = 42 as small uint").is_err());
}

#[test]
fn small_uint_uint_mismatch_is_error() {
    assert!(eval_program("dec small uint x = 42 as uint").is_err());
}
