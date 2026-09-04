use crate::entry::FnEntry;

pub static PATH_SPLIT: FnEntry = FnEntry {
    signature: "path_split(path)",
    description: "splits a path into [parent, filename] as a two-element array",
    example: r#"get std::path::path_split

dec arr[string] parts = path_split("src/main.rs")?"#,
    expected_output: None,
    returns: "arr[string]",
    errors: None,
    see_also: &["path_split_extension", "path_parent", "path_filename"],
    since: Some("v2.1.0"),
};
