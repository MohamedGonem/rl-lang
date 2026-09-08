use crate::entry::FnEntry;

pub static REALPATH: FnEntry = FnEntry {
    signature: "realpath(path)",
    description: "resolves a path to its canonical form, following symlinks",
    example: r#"get std::fs::realpath

dec string real = realpath(".")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if the path cannot be resolved"),
    see_also: &["readlink", "symlink"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
