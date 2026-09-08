use crate::entry::FnEntry;

pub static WITH_EXEC_WITH_STDIN: FnEntry = FnEntry {
    signature: "with_exec_with_stdin(exe, cmd, input)",
    description: "runs a specific executable with the given string piped to its stdin, returns stdout",
    example: r#"get std::process::with_exec_with_stdin

dec string out = with_exec_with_stdin("cat", "", "hello world")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec_with_stdin", "with_exec"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
