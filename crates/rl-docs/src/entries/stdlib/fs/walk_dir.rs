use crate::entry::FnEntry;

pub static WALK_DIR: FnEntry = FnEntry {
    signature: "walk_dir(path)",
    description: "recursively lists all files and directories under the given path",
    example: r#"get std::fs::walk_dir

dec arr[string] all = walk_dir("src")?"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some("Will return error if a directory cannot be read"),
    see_also: &["list_dir", "glob"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
