use crate::entry::FnEntry;

pub static EXEC_WITH_ENV: FnEntry = FnEntry {
    signature: "exec_with_env(cmd, envs)",
    description: "runs a shell command with custom environment variables, returns stdout",
    example: r#"get std::process::exec_with_env

dec string out = exec_with_env("env", [["MY_VAR", "hello"]])?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec", "with_exec_with_env"],
    since: None,
};
