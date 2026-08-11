use crate::entry::FnEntry;

pub static WITH_EXEC_LINES: FnEntry = FnEntry {
    signature: "with_exec_lines(exe, cmd)",
    description: "runs an executable directly (without the platform shell), splitting cmd into arguments, and returns its stdout split into an array of lines",
    example: r#"get std::process::with_exec_lines

dec arr[string] files = with_exec_lines("ls", "src")?"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some("Will return error on invalid arguments or failed command run"),
    see_also: &["with_exec", "with_exec_code"],
    since: Some("v0.4.0"),
};
