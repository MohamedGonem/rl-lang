use crate::entry::FnEntry;

pub static UNLOCK_FILE: FnEntry = FnEntry {
    signature: "unlock_file(path)",
    description: "releases an advisory lock on a file",
    example: r#"get std::fs::lock_file
get std::fs::unlock_file

lock_file("data.json")?
// ... safe file access ...
unlock_file("data.json")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the lock cannot be released"),
    see_also: &["lock_file"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
