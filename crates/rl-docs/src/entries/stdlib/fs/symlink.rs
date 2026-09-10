use crate::entry::FnEntry;

pub static SYMLINK: FnEntry = FnEntry {
    signature: "symlink(src, dst)",
    description: "creates a symbolic link from src to dst",
    example: r#"get std::fs::symlink

symlink("/tmp/original", "/tmp/link")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the symlink cannot be created"),
    see_also: &["readlink", "hardlink"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
