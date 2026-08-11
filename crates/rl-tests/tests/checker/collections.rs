use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn array_element_mismatch_errors() {
    assert_checker_msg(
        r#"dec arr[int] a = [1, "x"]"#,
        "type mismatch: array expects Int, got String",
    );
}

#[test]
fn array_decl_mismatch_errors() {
    assert_checker_msg(
        "dec arr[string] a = [1, 2]",
        "type mismatch: array expects String, got Int",
    );
}

#[test]
fn array_passes() {
    assert_checker_clean("dec arr[int] a = [1, 2]");
}
