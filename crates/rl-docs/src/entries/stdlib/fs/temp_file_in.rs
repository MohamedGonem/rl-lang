use crate::entry::FnEntry;

pub static TEMP_FILE_IN: FnEntry = FnEntry {
    signature: "temp_file_in(dir)",
    description: "creates a temporary file in the given directory and returns its path",
    example: r#"get std::fs::temp_file_in

dec string path = temp_file_in("/tmp/myapp")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the temp file cannot be created"),
    see_also: &["temp_file", "temp_dir"],
    since: None,
};
