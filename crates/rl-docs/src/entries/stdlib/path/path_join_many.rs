use crate::entry::FnEntry;

pub static PATH_JOIN_MANY: FnEntry = FnEntry {
    signature: "path_join_many(parts)",
    description: "joins an array of path segments into a single path",
    example: r#"get std::path::path_join_many

dec string full = path_join_many(["src", "utils", "helper.rs"])??"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["path_join", "path_components"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
