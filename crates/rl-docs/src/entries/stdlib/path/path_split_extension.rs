use crate::entry::FnEntry;

pub static PATH_SPLIT_EXTENSION: FnEntry = FnEntry {
    signature: "path_split_extension(path)",
    description: "splits a path into [stem, extension] as a two-element array (extension includes the dot)",
    example: r#"get std::path::path_split_extension

dec arr[string] parts = path_split_extension("main.rs")?"#,
    expected_output: None,
    returns: "arr[string]",
    errors: None,
    see_also: &["path_split", "path_extension", "path_stem"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
