use crate::common;
use rl_vm::VmValue;

#[test]
fn logical_and_truth_table() {
    let result = common::compile_and_run(
        r#"
        dec bool a = true and true
        dec bool b = true and false
        dec bool c = false and true
        dec bool d = false and false
        a == true and b == false and c == false and d == false
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn logical_or_truth_table() {
    let result = common::compile_and_run(
        r#"
        dec bool a = true or true
        dec bool b = true or false
        dec bool c = false or true
        dec bool d = false or false
        a == true and b == true and c == true and d == false
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Bool(true));
}

#[test]
fn logical_and_evaluates_right_side_when_left_is_true() {
    let err = common::compile_and_run(
        r#"
        dec bool x = true and (1 / 0 == 1)
        x
        "#,
    )
    .expect_err("right side of `and` should run and divide by zero");

    assert!(
        err.message().contains("division by zero"),
        "unexpected error message: {}",
        err.message()
    );
}

#[test]
fn logical_or_evaluates_right_side_when_left_is_false() {
    let err = common::compile_and_run(
        r#"
        dec bool x = false or (1 / 0 == 1)
        x
        "#,
    )
    .expect_err("right side of `or` should run and divide by zero");

    assert!(
        err.message().contains("division by zero"),
        "unexpected error message: {}",
        err.message()
    );
}
