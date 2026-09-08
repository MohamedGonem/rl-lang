use crate::entry::FnEntry;

pub static ISATTY: FnEntry = FnEntry {
    signature: "isatty()",
    description: "returns true if stdin is connected to a terminal",
    example: r#"get std::io::isatty

dec bool terminal = isatty()?"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
