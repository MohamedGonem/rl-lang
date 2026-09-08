use crate::entry::FnEntry;

pub static PATH_CANONICALIZE: FnEntry = FnEntry {
    signature: "path_canonicalize(path)",
    description: "resolves a path to its canonical form, following symlinks and removing . and ..",
    example: r#"get std::fs::path_canonicalize

dec string real = path_canonicalize(".")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if path does not exist"),
    see_also: &["path_absolute", "path_normalize"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
