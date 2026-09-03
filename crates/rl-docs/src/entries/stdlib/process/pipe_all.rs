use crate::entry::FnEntry;

pub static PIPE_ALL: FnEntry = FnEntry {
    signature: "pipe_all(cmds)",
    description: "chains multiple shell commands together like a unix pipeline, returns final stdout",
    example: r#"get std::process::pipe_all

dec string out = pipe_all(["echo hello world", "tr ' ' '\\n'", "sort"])??"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["pipe", "exec"],
    since: None,
};
