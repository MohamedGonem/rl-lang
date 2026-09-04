use crate::entry::FnEntry;

pub static PARENT_PID: FnEntry = FnEntry {
    signature: "parent_pid()",
    description: "returns the parent process id",
    example: r#"get std::process::parent_pid

dec int ppid = parent_pid()?"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["pid", "process_exists"],
    since: Some("v2.1.0"),
};
