use crate::entry::FnEntry;

pub static PATH_NORMALIZE: FnEntry = FnEntry {
    signature: "path_normalize(path)",
    description: "normalizes a path by collapsing . and .. segments and removing extra separators",
    example: r#"get std::path::path_normalize

dec string norm = path_normalize("src/../src/./main.rs")?"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["path_canonicalize", "path_absolute"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
