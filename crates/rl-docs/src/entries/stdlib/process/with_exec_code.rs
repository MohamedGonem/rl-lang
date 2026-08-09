use crate::entry::FnEntry;

pub static WITH_EXEC_CODE: FnEntry = FnEntry {
    signature: "with_exec_code(exe, cmd)",
    description: "runs an executable directly (without the platform shell), splitting cmd into arguments, and returns its exit code as an int",
    example: r#"get std::process::with_exec_code

dec int code = with_exec_code("ls", "/nonexistent")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error on invalid arguments or failed command run"),
    see_also: &["with_exec", "with_exec_lines"],
    since: Some("v0.4.0"),
};
