use rl_vm::VmValue;

use crate::common;

#[test]
fn explicit_entry_returns_its_value() {
    let result = common::compile_and_run(
        r#"
!#[entry]
fn start() { return 42 }
"#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn main_is_entry_when_present() {
    let result = common::compile_and_run(
        r#"
fn main() { return 7 }
"#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn entry_mode_skips_top_level_expressions() {
    let result = common::compile_and_run(
        r#"
dec int x = 0
x = 99
fn main() { return x }
"#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(0));
}

#[test]
fn inits_run_in_priority_order_before_entry() {
    let result = common::compile_and_run(
        r#"
dec int x = 0
!#[init=2]
fn init_two() { x = x * 10 + 2 }
!#[init=1]
fn init_one() { x = x * 10 + 1 }
!#[init]
fn init_default() { x = x * 10 + 3 }
!#[entry]
fn start() { return x }
"#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(123));
}

#[test]
fn finals_and_tests_run_without_error() {
    let result = common::compile_and_run(
        r#"
dec int x = 0
!#[entry]
fn start() { x = x + 10 }
!#[final=1]
fn final_one() { x = x + 1 }
!#[final]
fn final_default() { x = x + 2 }
!#[test]
fn check() { x = x + 100 }
"#,
    )
    .expect("vm run failed");
    assert_eq!(result, VmValue::Int(110));
}

#[test]
fn multiple_entry_functions_error() {
    let err = common::compile_error(
        r#"
!#[entry]
fn a() { return 1 }
!#[entry]
fn b() { return 2 }
"#,
    );
    assert!(
        err.contains("multiple !#[entry] functions found"),
        "unexpected error message: {err}"
    );
}
