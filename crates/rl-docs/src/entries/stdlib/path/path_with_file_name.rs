use crate::entry::FnEntry;

pub static PATH_WITH_FILE_NAME: FnEntry = FnEntry {
    signature: "path_with_file_name(path, name)",
    description: "returns a new path with the final component replaced by the given name",
    example: r#"get std::path::path_with_file_name

dec string new = path_with_file_name("src/main.rs", "lib.rs")?"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["path_set_extension", "path_filename"],
    since: Some("v2.1.0"),
};
