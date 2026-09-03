use crate::entry::FnEntry;

pub static EXEC_WITH_TIMEOUT: FnEntry = FnEntry {
    signature: "exec_with_timeout(cmd, timeout_ms)",
    description: "runs a shell command that must complete within the given timeout in milliseconds",
    example: r#"get std::process::exec_with_timeout

dec string out = exec_with_timeout("echo done", 5000)?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on timeout or failed command run"),
    see_also: &["exec", "exec_background"],
    since: None,
};
