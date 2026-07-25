use std::path::PathBuf;
use std::process::Command;

use crate::{
    evaluator::Evaluator,
    values::Value,
};

#[cfg(target_os = "windows")]
const SHARED_LIB_EXT: &str = "dll";
#[cfg(target_os = "macos")]
const SHARED_LIB_EXT: &str = "dylib";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
const SHARED_LIB_EXT: &str = "so";

fn compiler_command(compiler: &str, src: &PathBuf, out: &PathBuf) -> Command {
    let mut cmd = Command::new(compiler);
    #[cfg(target_os = "macos")]
    {
        cmd.args(["-dynamiclib", "-O2", "-o"]).arg(out).arg(src);
    }
    #[cfg(not(target_os = "macos"))]
    {
        cmd.args(["-shared", "-fPIC", "-O2", "-o"])
            .arg(out)
            .arg(src);
    }
    cmd
}

fn find_compiler() -> String {
    std::env::var("CC").unwrap_or_else(|_| "cc".to_string())
}

fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("rl_std_c_cache")
}

pub fn func(eval: &mut Evaluator, source: Value) -> Value {
    let source = match extract_string(source, "compile") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("compile: {}", e))),
    };
}
