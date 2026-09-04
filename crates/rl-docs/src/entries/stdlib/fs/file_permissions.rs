use crate::entry::FnEntry;

pub static FILE_PERMISSIONS: FnEntry = FnEntry {
    signature: "file_permissions(path)",
    description: "returns the permission mode bits of a file as an integer",
    example: r#"get std::fs::file_permissions

dec int mode = file_permissions("script.sh")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if permissions cannot be read"),
    see_also: &["set_permissions"],
    since: Some("v2.1.0"),
};
