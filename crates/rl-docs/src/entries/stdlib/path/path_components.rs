use crate::entry::FnEntry;

pub static PATH_COMPONENTS: FnEntry = FnEntry {
    signature: "path_components(path)",
    description: "splits a path into its constituent components as an array",
    example: r#"get std::path::path_components

dec arr[string] parts = path_components("src/utils/helper.rs")?"#,
    expected_output: None,
    returns: "arr[string]",
    errors: None,
    see_also: &["path_split", "path_join_many"],
    since: None,
};
