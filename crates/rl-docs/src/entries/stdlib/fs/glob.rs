use crate::entry::FnEntry;

pub static GLOB: FnEntry = FnEntry {
    signature: "glob(pattern)",
    description: "finds files matching a glob pattern and returns their paths",
    example: r#"get std::fs::glob

dec arr[string] files = glob("**/*.rs")?"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some("Will return error on invalid glob pattern"),
    see_also: &["walk_dir", "list_dir"],
    since: Some("v2.1.0"),
};
