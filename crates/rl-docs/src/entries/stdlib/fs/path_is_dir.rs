use crate::entry::FnEntry;

pub static PATH_IS_DIR: FnEntry = FnEntry {
    signature: "path_is_dir(path)",
    description: "returns true if the path is a directory",
    example: r#"get std::fs::path_is_dir

path_is_dir("./src")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["path_exists", "path_is_file"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
