use std::path::PathBuf;
use std::process::Command;

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

