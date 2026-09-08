use crate::entry::FnEntry;

pub static APPEND_FILE: FnEntry = FnEntry {
    signature: "append_file(path, content)",
    description: "appends content to a file, creating it if it does not exist",
    example: "get std::fs::append_file\n\nappend_file(\"info.txt\", \"name: Mohamed\")?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `path`'s parent directory does not exist\n- the current process lacks permission to write to `path`",
    ),
    see_also: &["write_file", "read_file"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
