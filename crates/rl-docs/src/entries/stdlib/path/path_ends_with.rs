use crate::entry::FnEntry;

pub static PATH_ENDS_WITH: FnEntry = FnEntry {
    signature: "path_ends_with(path, child)",
    description: "returns true if the path structurally ends with the given child path",
    example: r#"get std::path::path_ends_with

dec bool ends = path_ends_with("src/main.rs", "main.rs")?"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["path_starts_with"],
    since: None,
};
