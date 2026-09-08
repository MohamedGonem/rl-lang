use crate::entry::FnEntry;

pub static PATH_EXPAND_HOME: FnEntry = FnEntry {
    signature: "path_expand_home(path)",
    description: "expands ~ at the start of a path to the user's home directory",
    example: r#"get std::path::path_expand_home

dec string home = path_expand_home("~/Documents")?"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["path_absolute"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
