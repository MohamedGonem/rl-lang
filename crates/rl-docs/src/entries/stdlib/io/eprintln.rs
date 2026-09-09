use crate::entry::FnEntry;

pub static EPRINTLN: FnEntry = FnEntry {
    signature: "eprintln(message, ..)",
    description: "prints any value to stderr with trailing newline",
    example: "get std::io::eprintln\n\neprintln(\"something went wrong\")",
    expected_output: Some("something went wrong"),
    returns: "null",
    errors: None,
    see_also: &["eprint", "print", "println"],
    since: Some("v1.1.0"),
    deprecated: None,
    updated: Some("v1.1.0"),
};
