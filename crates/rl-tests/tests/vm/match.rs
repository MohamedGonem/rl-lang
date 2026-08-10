use crate::common;
use rl_vm::VmValue;

#[test]
fn match_matching_literal_arm_runs_its_body() {
    let result = common::compile_and_run(
        r#"
        dec int x = 2
        dec int res = 0
        match x {
            1 => { res = 100 }
            2 => { res = 200 }
            _ => { res = 300 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(200));
}

#[test]
fn match_first_arm_skips_remaining_arms() {
    let result = common::compile_and_run(
        r#"
        dec int x = 1
        dec int res = 0
        match x {
            1 => { res= 1 }
            1 => { res= 2 }
            _ => { res= 3 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn match_wildcard_is_fallback_for_unmatched_value() {
    let result = common::compile_and_run(
        r#"
        dec int x = 99
        dec int res = 0
        match x {
            1 => { res = 100 }
            2 => { res = 200 }
            _ => { res = 300 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(300));
}

#[test]
fn match_no_arm_matches_leaves_state_unchanged() {
    let result = common::compile_and_run(
        r#"
        dec int x = 99
        dec int res = 42
        match x {
            1 => { res = 100 }
            2 => { res = 200 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(42));
}

#[test]
fn match_arm_body_reads_outer_local() {
    let result = common::compile_and_run(
        r#"
        dec int factor = 10
        dec int x = 2
        dec int res = 0
        match x {
            1 => { res = factor }
            2 => { res = factor * 2 }
            _ => { res = factor }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(20));
}

#[test]
fn match_arm_body_reads_global() {
    let result = common::compile_and_run(
        r#"
        dec int factor = 7
        dec int x = 2
        dec int res = 0
        match x {
            2 => { res = factor }
            _ => { res = 0 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(7));
}

#[test]
fn match_arms_assign_to_outer_scope() {
    let result = common::compile_and_run(
        r#"
        dec int res = 0
        match 5 {
            5 => { res = res + 1 }
            _ => { res = 999 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(1));
}

#[test]
fn match_on_string_value() {
    let result = common::compile_and_run(
        r#"
        dec string s = "hi"
        dec int res = 0
        match s {
            "hi" => { res = 1 }
            "bye" => { res = 2 }
            _ => { res = 3 }
        }
        res
        "#,
    )
    .expect("vm run failed");

    assert_eq!(result, VmValue::Int(1));
}
