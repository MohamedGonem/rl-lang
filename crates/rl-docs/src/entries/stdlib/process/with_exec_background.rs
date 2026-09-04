use crate::entry::FnEntry;

pub static WITH_EXEC_BACKGROUND: FnEntry = FnEntry {
    signature: "with_exec_background(exe, cmd)",
    description: "spawns a specific executable in the background and returns its process id",
    example: r#"get std::process::with_exec_background

dec int pid = with_exec_background("sleep", "5")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error on failed process spawn"),
    see_also: &["exec_background", "wait_pid"],
    since: Some("v2.1.0"),
};
