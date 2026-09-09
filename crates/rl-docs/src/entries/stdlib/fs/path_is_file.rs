use crate::entry::FnEntry;

pub static PATH_IS_FILE: FnEntry = FnEntry {
    signature: "path_is_file(path)",
    description: "returns true if the path is a file",
    example: r#"get std::fs::path_is_file

path_is_file("./Cargo.toml")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["path_exists", "path_is_dir"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
