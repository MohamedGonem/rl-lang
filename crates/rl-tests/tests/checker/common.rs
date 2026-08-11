use crate::common::checker_messages;

pub fn assert_checker_clean(source: &str) {
    let msgs = checker_messages(source);
    assert!(msgs.is_empty(), "expected no errors, got: {msgs:?}");
}
pub fn assert_checker_msg(source: &str, fragment: &str) {
    let msgs = checker_messages(source);
    assert!(
        msgs.iter().any(|m| m.contains(fragment)),
        "expected a message containing {fragment:?}, got: {msgs:?}"
    );
}
