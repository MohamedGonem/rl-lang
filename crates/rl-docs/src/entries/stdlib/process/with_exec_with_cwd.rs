use crate::entry::FnEntry;

pub static WITH_EXEC_WITH_CWD: FnEntry = FnEntry {
    signature: "with_exec_with_cwd(exe, cmd, dir)",
    description: "runs a specific executable in the given working directory, returns stdout",
    example: r#"get std::process::with_exec_with_cwd

dec string out = with_exec_with_cwd("ls", "", "/tmp")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec_with_cwd", "with_exec"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
