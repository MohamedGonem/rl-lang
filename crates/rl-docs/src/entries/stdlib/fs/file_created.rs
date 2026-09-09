use crate::entry::FnEntry;

pub static FILE_CREATED: FnEntry = FnEntry {
    signature: "file_created(path)",
    description: "returns the creation time of a file as a unix timestamp in seconds",
    example: r#"get std::fs::file_created

dec int ts = file_created("Cargo.toml")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if creation time is not available"),
    see_also: &["file_modified", "file_accessed"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
