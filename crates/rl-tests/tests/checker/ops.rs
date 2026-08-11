use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn binary_mismatch_errors() {
    assert_checker_msg(
        "dec int x = 5\nx + 1.5",
        "type mismatch on +: got Int and Float",
    );
}

#[test]
fn and_needs_bool_errors() {
    assert_checker_msg("1 and 2", "expected bool on the left side of");
}

#[test]
fn bang_non_bool_errors() {
    assert_checker_msg("!5", "type mismatch on !: got Int");
}

#[test]
fn negate_string_errors() {
    assert_checker_msg(r#"-"hi""#, "type mismatch on unary -: got String");
}

#[test]
fn index_non_array_errors() {
    assert_checker_msg(
        "dec int x = 5\nx[0]",
        "invalid index operation: this is Int",
    );
}

#[test]
fn index_bad_index_type_errors() {
    assert_checker_msg(
        "dec arr[int] a = [1, 2]\na[\"x\"]",
        "invalid index operation: index is String",
    );
}

#[test]
fn index_assign_non_container_errors() {
    assert_checker_msg("dec int x = 5\nx[0] = 1", "cannot index-assign into Int");
}

#[test]
fn index_assign_value_mismatch_errors() {
    assert_checker_msg(
        "dec arr[int] a = [1, 2]\na[0] = \"x\"",
        "type mismatch: array is Int, cannot assign String",
    );
}

#[test]
fn index_assign_index_type_errors() {
    assert_checker_msg(
        "dec arr[int] a = [1, 2]\na[\"x\"] = 5",
        "index must be int, got String",
    );
}

#[test]
fn valid_index_passes() {
    assert_checker_clean("dec arr[int] a = [1, 2]\ndec int x = a[0]");
}
