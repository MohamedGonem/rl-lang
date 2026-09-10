use crate::entry::FnEntry;

pub static READLINK: FnEntry = FnEntry {
    signature: "readlink(path)",
    description: "returns the target of a symbolic link",
    example: r#"get std::fs::readlink

dec string target = readlink("/tmp/link")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the path is not a symlink"),
    see_also: &["symlink", "realpath"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
