use crate::entry::FnEntry;

pub static PATH_STARTS_WITH: FnEntry = FnEntry {
    signature: "path_starts_with(path, base)",
    description: "returns true if the path structurally starts with the given base path",
    example: r#"get std::path::path_starts_with

dec bool starts = path_starts_with("src/main.rs", "src")?"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["path_ends_with"],
    since: None,
};
