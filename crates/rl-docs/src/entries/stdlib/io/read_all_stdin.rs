use crate::entry::FnEntry;

pub static READ_ALL_STDIN: FnEntry = FnEntry {
    signature: "read_all_stdin()",
    description: "reads all of stdin until EOF and returns it as a string",
    example: r#"get std::io::read_all_stdin

dec string all = read_all_stdin()?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if stdin cannot be read"),
    see_also: &["read", "read_all"],
    since: Some("v2.1.0"),
};
