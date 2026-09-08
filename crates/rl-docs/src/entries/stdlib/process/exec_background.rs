use crate::entry::FnEntry;

pub static EXEC_BACKGROUND: FnEntry = FnEntry {
    signature: "exec_background(cmd)",
    description: "spawns a shell command in the background and returns its process id",
    example: r#"get std::process::exec_background

dec int pid = exec_background("sleep 5 & echo started")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error on failed process spawn"),
    see_also: &["wait_pid", "term_pid", "kill_pid"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
