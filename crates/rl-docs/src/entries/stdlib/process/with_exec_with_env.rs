use crate::entry::FnEntry;

pub static WITH_EXEC_WITH_ENV: FnEntry = FnEntry {
    signature: "with_exec_with_env(exe, cmd, envs)",
    description: "runs a specific executable with custom environment variables, returns stdout",
    example: r#"get std::process::with_exec_with_env

dec string out = with_exec_with_env("env", "", [["MY_VAR", "hello"]])?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec_with_env", "with_exec"],
    since: None,
};
