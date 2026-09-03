use crate::entry::FnEntry;

pub static PATH_IS_RELATIVE: FnEntry = FnEntry {
    signature: "path_is_relative(path)",
    description: "returns true if the path is relative (does not start from root)",
    example: r#"get std::path::path_is_relative

dec bool rel = path_is_relative("src/main.rs")?"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["path_is_absolute"],
    since: None,
};
