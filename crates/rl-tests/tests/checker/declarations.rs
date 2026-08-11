use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn decl_type_mismatch_errors() {
    assert_checker_msg(
        r#"dec int x = "hi""#,
        "type mismatch: expected Int, got String",
    );
}

#[test]
fn correct_decl_passes() {
    assert_checker_clean("dec int x = 5");
}

#[test]
fn const_type_mismatch_errors() {
    assert_checker_msg(
        "CONST string s = 5",
        "type mismatch: expected CString, got CInt",
    );
}

#[test]
fn const_reassign_errors() {
    assert_checker_msg("CONST int x = 5\nx = 6", "cannot assign to constant 'x'");
}

#[test]
fn const_redeclare_errors() {
    assert_checker_msg("CONST int x = 5\nCONST int x = 6", "is already declared");
}

#[test]
fn reassign_type_mismatch_errors() {
    assert_checker_msg(
        "dec int x = 5\nx = \"hi\"",
        "cannot assign String to variable 'x' declared as Int",
    );
}

#[test]
fn assign_undefined_errors() {
    assert_checker_msg("x = 5", "undefined variable 'x'");
}
