use crate::entry::FnEntry;

pub static ENV_KEYS: FnEntry = FnEntry {
    signature: "env_keys()",
    description: "returns an array of all environment variable names",
    example: r#"get std::process::env_keys

dec arr[string] keys = env_keys()?"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: None,
    see_also: &["env", "set_env"],
    since: None,
};
