use super::common::{assert_checker_clean, assert_checker_msg};

#[test]
fn tag_passes() {
    assert_checker_clean("tag C { Red }\ndec C c = C.Red");
}

#[test]
fn unknown_variant_errors() {
    assert_checker_msg("tag C { Red }\nC.Blue", "tag `C` has no variant `Blue`");
}

#[test]
fn match_pattern_mismatch_errors() {
    assert_checker_msg(
        "tag C { Red, Green }\ndec C c = C.Red\nmatch c { 5 => { } _ => { } }",
        "match pattern type does not match value type",
    );
}
