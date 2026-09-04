use crate::entry::FnEntry;

pub static PROCESS_EXISTS: FnEntry = FnEntry {
    signature: "process_exists(pid)",
    description: "returns true if a process with the given pid exists",
    example: r#"get std::process::exec_background
get std::process::process_exists

dec int pid = exec_background("sleep 5")?
dec bool alive = process_exists(pid)?"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["pid", "parent_pid", "wait_pid"],
    since: Some("v2.1.0"),
};
