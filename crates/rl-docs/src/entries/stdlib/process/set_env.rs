use crate::entry::FnEntry;

pub static SET_ENV: FnEntry = FnEntry {
    signature: "set_env(key, value)",
    description: "sets an environment variable for the current process",
    example: r#"get std::process::set_env

set_env("MY_VAR", "hello")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: None,
    see_also: &["env", "remove_env"],
    since: Some("v2.1.0"),
};
