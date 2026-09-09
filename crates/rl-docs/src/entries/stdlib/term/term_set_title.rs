use crate::entry::FnEntry;

pub static TERM_SET_TITLE: FnEntry = FnEntry {
    signature: "term_set_title(title)",
    description: "sets the terminal window title",
    example: r#"get std::term::term_set_title

term_set_title("My App")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `title` is not a string
- writing to stdout fails"#,
    ),
    see_also: &[],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
