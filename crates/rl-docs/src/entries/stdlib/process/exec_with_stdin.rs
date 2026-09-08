use crate::entry::FnEntry;

pub static EXEC_WITH_STDIN: FnEntry = FnEntry {
    signature: "exec_with_stdin(cmd, input)",
    description: "runs a shell command with the given string piped to its stdin, returns stdout",
    example: r#"get std::process::exec_with_stdin

dec string out = exec_with_stdin("cat", "hello world")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec", "with_exec_with_stdin"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
