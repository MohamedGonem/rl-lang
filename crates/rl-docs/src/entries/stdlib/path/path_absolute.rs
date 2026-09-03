use crate::entry::FnEntry;

pub static PATH_ABSOLUTE: FnEntry = FnEntry {
    signature: "path_absolute(path)",
    description: "resolves a relative path to an absolute path using the current working directory",
    example: r#"get std::path::path_absolute

dec string abs = path_absolute("src/main.rs")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if current working directory cannot be determined"),
    see_also: &["path_canonicalize", "path_relative"],
    since: None,
};
