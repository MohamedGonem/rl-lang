use crate::entry::FnEntry;

pub static EXEC_WITH_CWD: FnEntry = FnEntry {
    signature: "exec_with_cwd(cmd, dir)",
    description: "runs a shell command in the given working directory, returns stdout",
    example: r#"get std::process::exec_with_cwd

dec string out = exec_with_cwd("ls", "/tmp")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec", "with_exec_with_cwd"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
