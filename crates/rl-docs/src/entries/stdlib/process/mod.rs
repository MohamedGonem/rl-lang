use crate::entry::{FnEntry, StdEntry};

mod args;
mod cwd;
mod env;
mod exec;
mod exec_code;
mod exec_lines;
mod exit;
mod pid;
mod set_cwd;
mod sleep;
mod with_exec;
mod with_exec_code;
mod with_exec_lines;

pub static PROCESS: StdEntry = StdEntry {
    name: "process",
    description: "functions for interacting with the current process and running shell commands",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &args::ARGS,
    &cwd::CWD,
    &env::ENV,
    &exec::EXEC,
    &exec_code::EXEC_CODE,
    &exec_lines::EXEC_LINES,
    &exit::EXIT,
    &pid::PID,
    &set_cwd::SET_CWD,
    &sleep::SLEEP,
    &with_exec::WITH_EXEC,
    &with_exec_code::WITH_EXEC_CODE,
    &with_exec_lines::WITH_EXEC_LINES,
];
