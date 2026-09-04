use crate::entry::FnEntry;

pub static TOUCH: FnEntry = FnEntry {
    signature: "touch(path)",
    description: "creates an empty file if it does not exist, or updates its timestamp",
    example: r#"get std::fs::touch

touch("new_file.txt")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the file cannot be created"),
    see_also: &["write_file", "create_file"],
    since: Some("v2.1.0"),
};
