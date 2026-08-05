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
