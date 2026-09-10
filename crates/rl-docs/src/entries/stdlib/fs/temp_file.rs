use crate::entry::FnEntry;

pub static TEMP_FILE: FnEntry = FnEntry {
    signature: "temp_file()",
    description: "creates a temporary file and returns its path",
    example: r#"get std::fs::temp_file

dec string path = temp_file()?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the temp file cannot be created"),
    see_also: &["temp_file_in", "temp_dir"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
