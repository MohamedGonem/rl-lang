use crate::entry::FnEntry;

pub static TRUNCATE_FILE: FnEntry = FnEntry {
    signature: "truncate_file(path, len)",
    description: "truncates or extends a file to the given length in bytes",
    example: r#"get std::fs::truncate_file

truncate_file("data.txt", 0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the file cannot be opened"),
    see_also: &["file_size", "write_file"],
    since: Some("v2.1.0"),
};
