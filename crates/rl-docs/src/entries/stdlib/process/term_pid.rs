use crate::entry::FnEntry;

pub static TERM_PID: FnEntry = FnEntry {
    signature: "term_pid(pid)",
    description: "sends SIGTERM to the given process for graceful shutdown",
    example: r#"get std::process::exec_background
get std::process::term_pid

dec int pid = exec_background("sleep 60")?
term_pid(pid)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the signal could not be sent"),
    see_also: &["kill_pid", "exec_background"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
