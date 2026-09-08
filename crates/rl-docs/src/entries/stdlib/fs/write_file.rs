use crate::entry::FnEntry;

pub static WRITE_FILE: FnEntry = FnEntry {
    signature: "write_file(path, contents)",
    description: "writes content to a file, overwriting it if it already exists",
    example: "get std::fs::write_file\n\nwrite_file(\"index.html\", \"<p>hello \\\"Mohamed\\\"</p>\")?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `path`'s parent directory does not exist\n- the current process lacks permission to write to `path`",
    ),
    see_also: &["append_file", "read_file"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
