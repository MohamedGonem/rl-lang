use crate::entry::FnEntry;

pub static PATH_IS_ABSOLUTE: FnEntry = FnEntry {
    signature: "path_is_absolute(path)",
    description: "returns true if the path is absolute (starts from root)",
    example: r#"get std::path::path_is_absolute

dec bool abs = path_is_absolute("/home/user")?"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["path_is_relative"],
    since: Some("v2.1.0"),
};
