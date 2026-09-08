use crate::entry::FnEntry;

pub static SET_PERMISSIONS: FnEntry = FnEntry {
    signature: "set_permissions(path, mode)",
    description: "sets the permission mode bits of a file",
    example: r#"get std::fs::set_permissions

set_permissions("script.sh", 0o755)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if permissions cannot be set"),
    see_also: &["file_permissions"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
