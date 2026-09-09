use crate::entry::FnEntry;

pub static PATH_CANONICALIZE: FnEntry = FnEntry {
    signature: "path_canonicalize(path)",
    description: "resolves a path to its canonical form, following symlinks and removing . and ..",
    example: r#"get std::path::path_canonicalize

dec string real = path_canonicalize(".")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if path does not exist"),
    see_also: &["path_absolute", "path_normalize"],
    since: Some("v2.1.0"),
    deprecated: Some("moved to std::fs::path_canonicalize"),
    updated: Some("v2.1.0"),
};
