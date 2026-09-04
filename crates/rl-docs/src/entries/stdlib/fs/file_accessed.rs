use crate::entry::FnEntry;

pub static FILE_ACCESSED: FnEntry = FnEntry {
    signature: "file_accessed(path)",
    description: "returns the last access time of a file as a unix timestamp in seconds",
    example: r#"get std::fs::file_accessed

dec int ts = file_accessed("Cargo.toml")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if access time is not available"),
    see_also: &["file_modified", "file_created"],
    since: Some("v2.1.0"),
};
