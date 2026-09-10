use crate::entry::FnEntry;

pub static PATH_EXISTS: FnEntry = FnEntry {
    signature: "path_exists(path)",
    description: "returns true if the path exists on the filesystem",
    example: r#"get std::fs::path_exists

path_exists("./Cargo.toml")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["path_is_dir", "path_is_file"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
