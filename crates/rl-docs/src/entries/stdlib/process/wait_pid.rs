use crate::entry::FnEntry;

pub static WAIT_PID: FnEntry = FnEntry {
    signature: "wait_pid(pid)",
    description: "blocks until the given process exits and returns its exit code",
    example: r#"get std::process::exec_background
get std::process::wait_pid

dec int pid = exec_background("echo done && exit 42")?
dec int code = wait_pid(pid)?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if pid is not a tracked background process"),
    see_also: &["exec_background", "kill_pid"],
    since: None,
};
