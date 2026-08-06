use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
pub fn docs_dir() -> PathBuf {
    PathBuf::from(std::env::var_os("APPDATA").unwrap())
        .join("rl")
        .join("docs")
}

#[cfg(unix)]
pub fn docs_dir() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap())
        .join(".rl")
        .join("docs")
}

pub fn open_browser(path: &Path) -> std::io::Result<()> {
    open(path)
}

#[cfg(target_os = "windows")]
fn open(path: &Path) -> std::io::Result<()> {
    Command::new("cmd")
        .args([
            "/C",
            "start",
            "",
            path.to_str().unwrap(),
        ])
        .spawn()?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn open(path: &Path) -> std::io::Result<()> {
    Command::new("xdg-open")
        .arg(path)
        .spawn()?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn open(path: &Path) -> std::io::Result<()> {
    Command::new("open")
        .arg(path)
        .spawn()?;

    Ok(())
}