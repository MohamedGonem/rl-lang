use crate::common;
use rl_vm::VmValue;

#[test]
fn basic_variable_arithemtic() {
    let result = common::compile_and_run(
        r#"
        1 + 2 * 4 * 4 + 342 * 1
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(375))
}

#[test]
fn uint_arithmetic() {
    let result = common::compile_and_run(r#"(1000 as uint) + (1 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::UInt(1001))
}

#[test]
fn uint_multiplication() {
    let result = common::compile_and_run(r#"(10 as uint) * (5 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::UInt(50))
}

#[test]
fn uint_division() {
    let result = common::compile_and_run(r#"(10 as uint) / (3 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::UInt(3))
}

#[test]
fn small_uint_arithmetic() {
    let result = common::compile_and_run(r#"(1000 as small uint) + (1 as small uint)"#)
        .expect("vm run failed");
    assert_eq!(result, VmValue::SUInt(1001))
}

#[test]
fn uint_comparison() {
    let result = common::compile_and_run(r#"(1 as uint) < (2 as uint)"#).expect("vm run failed");
    assert_eq!(result, VmValue::Bool(true))
}

#[test]
fn uint_overflow_is_error() {
    assert!(common::compile_and_run(r#"(18446744073709551615 as uint) + (1 as uint)"#).is_err());
}

#[test]
fn uint_underflow_is_error() {
    assert!(common::compile_and_run(r#"(1 as uint) - (2 as uint)"#).is_err());
}

#[test]
fn cast_value_to_int() {
    let result = common::compile_and_run("65.5 as int").expect("vm run failed");
    assert_eq!(result, VmValue::Int(65));
}

#[test]
fn cast_value_to_float() {
    let result = common::compile_and_run("3 as float").expect("vm run failed");
    assert_eq!(result, VmValue::Float(3.0));
}

#[test]
fn cast_value_to_byte() {
    let result = common::compile_and_run("200 as byte").expect("vm run failed");
    assert_eq!(result, VmValue::Byte(200));
}

#[test]
fn cast_value_to_small_int() {
    let result = common::compile_and_run("65536 as small int").expect("vm run failed");
    assert_eq!(result, VmValue::SInt(65536));
}

#[test]
fn cast_value_to_small_float() {
    let result = common::compile_and_run("7 as small float").expect("vm run failed");
    assert_eq!(result, VmValue::SFloat(7.0));
}

#[test]
fn units_are_discarded_by_the_vm() {
    let result = common::compile_and_run(
        r#"
        dec float distance: m = 100.0
        dec float time: s = 4.0
        dec float speed: m/s = distance / time
        speed * time
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Float(100.0));
}

#[test]
fn const_with_unit_runs_in_vm() {
    let result = common::compile_and_run(
        r#"
        CONST float SPEED: m/s = 12.5
        dec float time: s = 4.0
        SPEED * time
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Float(50.0));
}
