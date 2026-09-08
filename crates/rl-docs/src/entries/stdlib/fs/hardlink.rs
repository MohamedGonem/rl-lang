use crate::entry::FnEntry;

pub static HARDLINK: FnEntry = FnEntry {
    signature: "hardlink(src, dst)",
    description: "creates a hard link from src to dst",
    example: r#"get std::fs::hardlink

hardlink("original.txt", "linked.txt")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("Will return error if the hard link cannot be created"),
    see_also: &["symlink"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
