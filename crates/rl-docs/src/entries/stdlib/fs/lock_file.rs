use crate::entry::FnEntry;

pub static LOCK_FILE: FnEntry = FnEntry {
    signature: "lock_file(path)",
    description: "acquires an exclusive advisory lock on a file",
    example: r#"get std::fs::lock_file
get std::fs::unlock_file

lock_file("data.json")?
// ... safe file access ...
unlock_file("data.json")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the lock cannot be acquired"),
    see_also: &["unlock_file"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
