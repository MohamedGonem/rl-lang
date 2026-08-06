use rl_interpreter::values::Value;

use crate::common::eval_program;

#[test]
fn dec_int() {
    let evaluator = eval_program("dec int x = 42").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn dec_float() {
    let evaluator = eval_program("dec float x = 42.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(42.0)));
}

#[test]
fn const_int() {
    let evaluator = eval_program("CONST int x = 42").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(42)));
}

#[test]
fn const_float() {
    let evaluator = eval_program("CONST float x = 42.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(42.0)));
}

#[test]
fn assign_int() {
    let evaluator = eval_program("dec int x = 42\nx = 3").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(3)));
}

#[test]
fn assign_float() {
    let evaluator = eval_program("dec float x = 42.0\nx = 3.0").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(3.0)));
}

#[test]
fn compound_assign_int() {
    let evaluator = eval_program(
        r#"
dec int x = 42
x += 3"#,
    )
    .unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Integer(45)));
}

#[test]
fn compound_assign_float() {
    let evaluator = eval_program(
        r#"
dec float x = 42.0
x += 3.0"#,
    )
    .unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::Float(45.0)));
}

#[test]
fn int_assigned_float_is_error() {
    assert!(eval_program("dec int x = 1.0").is_err());
}

#[test]
fn float_assigned_int_is_error() {
    assert!(eval_program("dec float x = 1").is_err());
}

#[test]
fn const_int_reassigned_is_error() {
    assert!(eval_program("CONST int x = 1\nx = 2").is_err());
}

#[test]
fn const_float_reassigned_is_error() {
    assert!(eval_program("CONST float x = 1.0\nx = 2.0").is_err());
}

#[test]
fn int_undefined_variable_is_error() {
    assert!(eval_program("dec int x = y").is_err());
}

#[test]
fn float_undefined_variable_is_error() {
    assert!(eval_program("dec float x = y").is_err());
}

#[test]
fn const_int_undefined_variable_is_error() {
    assert!(eval_program("CONST int x = y").is_err());
}

#[test]
fn const_float_undefined_variable_is_error() {
    assert!(eval_program("CONST float x = y").is_err());
}

#[test]
fn dec_uint() {
    let evaluator = eval_program("dec uint x = 42 as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(42)));
}

#[test]
fn dec_small_uint() {
    let evaluator = eval_program("dec small uint x = 42 as small uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(42)));
}

#[test]
fn const_uint() {
    let evaluator = eval_program("CONST uint x = 42 as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(42)));
}

#[test]
fn const_small_uint() {
    let evaluator = eval_program("CONST small uint x = 42 as small uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(42)));
}

#[test]
fn assign_uint() {
    let evaluator = eval_program("dec uint x = 42 as uint\nx = 3 as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(3)));
}

#[test]
fn assign_small_uint() {
    let evaluator = eval_program("dec small uint x = 42 as small uint\nx = 3 as small uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(3)));
}

#[test]
fn compound_assign_uint() {
    let evaluator = eval_program("dec uint x = 42 as uint\nx += 3 as uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::UInteger(45)));
}

#[test]
fn compound_assign_small_uint() {
    let evaluator = eval_program("dec small uint x = 42 as small uint\nx += 3 as small uint").unwrap();
    assert_eq!(evaluator.get_value_raw("x"), Some(Value::SUInteger(45)));
}

#[test]
fn uint_plain_int_is_error() {
    assert!(eval_program("dec uint x = 42").is_err());
}

#[test]
fn uint_float_is_error() {
    assert!(eval_program("dec uint x = 42.0").is_err());
}

#[test]
fn uint_negative_is_error() {
    assert!(eval_program("dec uint x = -1 as uint").is_err());
}

#[test]
fn small_uint_negative_is_error() {
    assert!(eval_program("dec small uint x = -1 as small uint").is_err());
}

#[test]
fn const_uint_reassigned_is_error() {
    assert!(eval_program("CONST uint x = 1 as uint\nx = 2 as uint").is_err());
}

#[test]
fn const_small_uint_reassigned_is_error() {
    assert!(eval_program("CONST small uint x = 1 as small uint\nx = 2 as small uint").is_err());
}

#[test]
fn uint_undefined_variable_is_error() {
    assert!(eval_program("dec uint x = y").is_err());
}

#[test]
fn small_uint_undefined_variable_is_error() {
    assert!(eval_program("dec small uint x = y").is_err());
}

#[test]
fn uint_max_value() {
    let evaluator = eval_program("dec uint x = 18446744073709551615 as uint").unwrap();
    assert_eq!(
        evaluator.get_value_raw("x"),
        Some(Value::UInteger(u64::MAX))
    );
}

#[test]
fn small_uint_max_value() {
    let evaluator = eval_program("dec small uint x = 4294967295 as small uint").unwrap();
    assert_eq!(
        evaluator.get_value_raw("x"),
        Some(Value::SUInteger(u32::MAX))
    );
}
