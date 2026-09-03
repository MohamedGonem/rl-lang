use crate::entry::FnEntry;

pub static OS_NAME: FnEntry = FnEntry {
    signature: "os_name()",
    description: "returns the operating system name (linux, windows, macos)",
    example: r#"get std::process::os_name

dec string os = os_name()?"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["arch"],
    since: None,
};
