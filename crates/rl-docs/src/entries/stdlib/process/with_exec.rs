use crate::entry::FnEntry;

pub static WITH_EXEC: FnEntry = FnEntry {
    signature: "with_exec(exe, cmd)",
    description: "runs an executable directly (without the platform shell), splitting cmd into arguments, and returns its stdout as a trimmed string",
    example: r#"get std::process::with_exec

dec string out = with_exec("echo", "hello")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on invalid arguments or failed command run"),
    see_also: &["with_exec_code", "with_exec_lines"],
    since: Some("v0.4.0"),
};
