use crate::entry::FnEntry;

pub static REMOVE_ENV: FnEntry = FnEntry {
    signature: "remove_env(key)",
    description: "removes an environment variable from the current process",
    example: r#"get std::process::set_env
get std::process::remove_env

set_env("MY_VAR", "hello")?
remove_env("MY_VAR")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: None,
    see_also: &["env", "set_env"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
