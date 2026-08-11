use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn break_outside_loop_errors() {
    assert_checker_msg("break", "break outside of loop");
}

#[test]
fn continue_outside_loop_errors() {
    assert_checker_msg("continue", "continue outside of loop");
}

#[test]
fn break_in_loop_passes() {
    assert_checker_clean("while (true) { break }");
}

#[test]
fn continue_in_loop_passes() {
    assert_checker_clean("while (true) { continue }");
}

#[test]
fn return_outside_fn_errors() {
    assert_checker_msg("return 5", "return outside of function");
}

#[test]
fn return_wrong_type_errors() {
    assert_checker_msg(r#"fn f() -> int { return "hi" }"#, "return type mismatch");
}

#[test]
fn return_correct_passes() {
    assert_checker_clean("fn f() -> int { return 5 }");
}
