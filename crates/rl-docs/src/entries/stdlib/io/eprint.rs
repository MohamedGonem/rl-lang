use crate::entry::FnEntry;

pub static EPRINT: FnEntry = FnEntry {
    signature: "eprint(message, ..)",
    description: "prints any value to stderr without trailing newline",
    example: "get std::io::eprint\n\neprint(\"something went wrong\")",
    expected_output: Some("something went wrong"),
    returns: "null",
    errors: None,
    see_also: &["eprintln", "print", "println"],
    since: Some("v1.1.0"),
};
