use crate::entry::FnEntry;

pub static LIST_DIR_NAMES: FnEntry = FnEntry {
    signature: "list_dir_names(path)",
    description: "lists the names of entries in a directory without full paths",
    example: r#"get std::fs::list_dir_names

dec arr[string] names = list_dir_names("src")?"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some("Will return error if the directory cannot be read"),
    see_also: &["list_dir", "walk_dir"],
    since: Some("v2.1.0"),
};
