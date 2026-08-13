//! `std::process` - process management: args, env, cwd, exec, exit, pid, sleep.
//!
//! `exec` captures stdout and returns it as a string (trailing newline
//! stripped). `exec_code` returns only the exit code as `int`. `exec_lines`
//! returns stdout split into lines as `arr[string]`. The `with_*` variants take
//! an explicit executable plus a shell-word-split argument string (via the
//! `shell-words` crate) instead of running through the platform shell. `env`
//! returns `null` (not an error) when the variable is not set, so it builds a
//! raw `R::Value`; it is registered `untyped` (matching the canonical
//! `rl-commons` signature, which lists it via `with_functions(&["env"])`).
//!
//! Ported once from the former per-runtime `stdlib/process/*.rs` copies.

#[cfg(feature = "impls")]
use rl_std_core::Runtime;
use rl_std_macros::native_fn;
#[cfg(feature = "impls")]
use std::process::Command;
#[cfg(feature = "impls")]
use std::time::Duration;

// ---- args (no rl arguments, `array[string]`) ------------------------------

// NOTE: the original per-runtime `std_args` skipped `cx.user_args_offset`, a
// field on `Vm`/`Evaluator` that the CLI can override via
// `with_user_args_offset`. That field is NOT exposed through the `Runtime`
// trait, so this generic port cannot read it and falls back to the default
// offset of `1` (which matches both runtimes' default constructor). Preserving
// the configurable offset requires adding an accessor to `Runtime` (e.g.
// `fn user_args_offset(cx: &Self::Cx) -> usize`) and swapping the literal `1`
// below for `R::user_args_offset(cx)`.
#[native_fn(module = "process")]
pub fn args<R: Runtime>(cx: &mut R::Cx) -> Vec<String> {
    std::env::args().skip(R::user_args_offset(cx)).collect()
}

// ---- exit / pid / sleep ---------------------------------------------------

#[native_fn(module = "process")]
pub fn exit(code: i64) {
    std::process::exit(code as i32);
}

#[native_fn(module = "process")]
pub fn pid() -> i64 {
    std::process::id() as i64
}

#[native_fn(module = "process")]
pub fn sleep(ms: i64) {
    std::thread::sleep(Duration::from_millis(ms.max(0) as u64));
}

// ---- env (string-or-null, untyped, raw value) -----------------------------

#[native_fn(module = "process", untyped)]
pub fn env<R: Runtime>(key: String) -> R::Value {
    match std::env::var(&key) {
        Ok(val) => R::from_string(val),
        Err(_) => R::null(),
    }
}

// ---- cwd / set_cwd (language `result`) ------------------------------------

#[native_fn(module = "process")]
pub fn cwd() -> Result<String, String> {
    match std::env::current_dir() {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => Err(format!("cwd: {}", e)),
    }
}

#[native_fn(module = "process")]
pub fn set_cwd(path: String) -> Result<(), String> {
    match std::env::set_current_dir(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("set_cwd: failed to change to \"{}\": {}", path, e)),
    }
}

// ---- shell helpers --------------------------------------------------------

#[cfg(feature = "impls")]
#[cfg(target_os = "windows")]
fn shell_command(cmd: &str) -> Command {
    let mut c = Command::new("cmd");
    c.args(["/C", cmd]);
    c
}

#[cfg(feature = "impls")]
#[cfg(not(target_os = "windows"))]
fn shell_command(cmd: &str) -> Command {
    let mut c = Command::new("sh");
    c.args(["-c", cmd]);
    c
}

#[cfg(feature = "impls")]
fn with_command(e: &str, cmd: &str) -> Result<Command, shell_words::ParseError> {
    let args = shell_words::split(cmd)?;
    let mut c = Command::new(e);
    c.args(args);
    Ok(c)
}

// ---- exec (result[string]) ------------------------------------------------

#[native_fn(module = "process")]
pub fn exec(cmd: String) -> Result<String, String> {
    let output = match shell_command(&cmd).output() {
        Ok(o) => o,
        Err(e) => return Err(format!("exec: failed to run \"{}\": {}", cmd, e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

#[native_fn(module = "process")]
pub fn with_exec(e: String, cmd: String) -> Result<String, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => return Err(format!("with_exec: invalid args \"{}\": {}", cmd, err)),
    };
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec: failed to run \"{}\": {}", cmd, e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout.trim_end_matches('\n').to_string())
}

// ---- exec_code (result[int]) ----------------------------------------------

#[native_fn(module = "process")]
pub fn exec_code(cmd: String) -> Result<i64, String> {
    let status = match shell_command(&cmd).status() {
        Ok(s) => s,
        Err(e) => return Err(format!("exec_code: failed to run \"{}\": {}", cmd, e)),
    };
    Ok(status.code().unwrap_or(-1) as i64)
}

#[native_fn(module = "process")]
pub fn with_exec_code(e: String, cmd: String) -> Result<i64, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => return Err(format!("with_exec: invalid args \"{}\": {}", cmd, err)),
    };
    let status = match command.status() {
        Ok(s) => s,
        Err(e) => return Err(format!("with_exec_code: failed to run \"{}\": {}", cmd, e)),
    };
    Ok(status.code().unwrap_or(-1) as i64)
}

// ---- exec_lines (result[array[string]]) -----------------------------------

#[native_fn(module = "process")]
pub fn exec_lines(cmd: String) -> Result<Vec<String>, String> {
    let output = match shell_command(&cmd).output() {
        Ok(o) => o,
        Err(e) => return Err(format!("exec_lines: failed to run \"{}\": {}", cmd, e)),
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<String> = stdout.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

#[native_fn(module = "process")]
pub fn with_exec_lines(e: String, cmd: String) -> Result<Vec<String>, String> {
    let mut command = match with_command(&e, &cmd) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!(
                "with_exec_lines: invalid args \"{}\": {}",
                cmd, err
            ));
        }
    };
    let output = match command.output() {
        Ok(o) => o,
        Err(e) => return Err(format!("with_exec_lines: failed to run \"{}\": {}", cmd, e)),
    };

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<String> = stdout.lines().map(|l| l.to_string()).collect();
    Ok(lines)
}

rl_std_core::native_module!("process";
    funcs: [
        args,
        exit, pid, sleep,
        env,
        cwd, set_cwd,
        exec, with_exec,
        exec_code, with_exec_code,
        exec_lines, with_exec_lines,
    ],
);
