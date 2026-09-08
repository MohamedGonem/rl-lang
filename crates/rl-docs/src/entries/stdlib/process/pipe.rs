use crate::entry::FnEntry;

pub static PIPE: FnEntry = FnEntry {
    signature: "pipe(cmd1, cmd2)",
    description: "chains two shell commands together like a unix pipe, returns final stdout",
    example: r#"get std::process::pipe

dec string out = pipe("echo hello world", "wc -w")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["pipe_all", "exec"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
