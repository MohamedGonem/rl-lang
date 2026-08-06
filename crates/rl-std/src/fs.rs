//! `std::fs` - filesystem operations: create, remove, list, copy, move, and
//! metadata.
//!
//! All errors include the path and the underlying OS error message. Every
//! fallible function returns a language `result[T]` (an `Ok`/`Err` value):
//! `mkdir`/`mkdir_all`/`rmdir`/`rmdir_all`/`move_file` yield `result[null]`,
//! `copy_file`/`file_size`/`file_modified` yield `result[int]`, `list_dir`
//! yields `result[array[string]]`, and `rename_file` yields `result[string]`.
//! `temp_dir` takes no arguments and returns a plain string. Ported once from
//! the former per-runtime `stdlib/fs/*.rs` copies.

use rl_std_macros::native_fn;

// ---- directory creation / removal (language `result[null]`) ---------------

#[native_fn(module = "fs")]
pub fn mkdir(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::create_dir(&path) {
        return Err(format!("mkdir: failed to create \"{}\": {}", path, e));
    };
    Ok(())
}

#[native_fn(module = "fs")]
pub fn mkdir_all(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::create_dir_all(&path) {
        return Err(format!("mkdir_all: failed to create \"{}\": {}", path, e));
    };
    Ok(())
}

#[native_fn(module = "fs")]
pub fn rmdir(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::remove_dir(&path) {
        return Err(format!("rmdir: failed to delete \"{}\": {}", path, e));
    };
    Ok(())
}

#[native_fn(module = "fs")]
pub fn rmdir_all(path: String) -> Result<(), String> {
    if let Err(e) = std::fs::remove_dir_all(&path) {
        return Err(format!("rmdir_all: failed to delete \"{}\": {}", path, e));
    };
    Ok(())
}

// ---- listing (language `result[array[string]]`) ---------------------------

#[native_fn(module = "fs")]
pub fn list_dir(path: String) -> Result<Vec<String>, String> {
    match std::fs::read_dir(&path) {
        Err(e) => Err(format!("list_dir: failed to read \"{}\": {}", path, e)),
        Ok(d) => Ok(d
            .filter_map(|i| i.ok())
            .map(|i| i.path().to_string_lossy().to_string())
            .collect::<Vec<String>>()),
    }
}

// ---- copy / move ----------------------------------------------------------

#[native_fn(module = "fs")]
pub fn copy_file(src: String, dst: String) -> Result<i64, String> {
    let bytes = match std::fs::copy(&src, &dst) {
        Ok(b) => b,
        Err(e) => {
            return Err(format!(
                "copy_file: failed to copy \"{}\" to \"{}\": {}",
                src, dst, e
            ));
        }
    };
    Ok(bytes as i64)
}

#[native_fn(module = "fs")]
pub fn move_file(src: String, dst: String) -> Result<(), String> {
    if let Err(e) = std::fs::rename(&src, &dst) {
        return Err(format!(
            "move_file: failed to move \"{}\" to \"{}\": {}",
            src, dst, e
        ));
    };
    Ok(())
}

// ---- metadata (language `result[int]`) ------------------------------------

#[native_fn(module = "fs")]
pub fn file_size(path: String) -> Result<i64, String> {
    match std::fs::metadata(&path) {
        Err(e) => Err(format!("file_size: failed to read \"{}\": {}", path, e)),
        Ok(metadata) => Ok(metadata.len() as i64),
    }
}

#[native_fn(module = "fs")]
pub fn file_modified(path: String) -> Result<i64, String> {
    match std::fs::metadata(&path) {
        Err(e) => Err(format!("file_modified: failed to read \"{}\": {}", path, e)),

        Ok(metadata) => match metadata.modified() {
            Err(e) => Err(format!(
                "file_modified: could not get modification time for \"{}\": {}",
                path, e
            )),
            Ok(modified) => match modified.duration_since(std::time::UNIX_EPOCH) {
                Err(e) => Err(format!(
                    "file_modified: modification time before epoch for \"{}\": {}",
                    path, e
                )),
                Ok(t) => Ok(t.as_secs() as i64),
            },
        },
    }
}

// ---- temp dir (plain string, no arguments) --------------------------------

#[native_fn(module = "fs")]
pub fn temp_dir() -> String {
    std::env::temp_dir().to_string_lossy().to_string()
}

// ---- rename (language `result[string]`) -----------------------------------

#[native_fn(module = "fs")]
pub fn rename_file(path: String, new_name: String) -> Result<String, String> {
    let old_path = std::path::Path::new(&path);
    let new_path = match old_path.parent() {
        Some(parent) => parent.join(&new_name),
        None => std::path::PathBuf::from(&new_name),
    };

    if let Err(e) = std::fs::rename(old_path, &new_path) {
        return Err(format!(
            "rename_file(): failed to rename \"{}\" to \"{}\": {}",
            path,
            new_path.to_string_lossy(),
            e
        ));
    };

    Ok(new_path.to_string_lossy().to_string())
}

rl_std_core::native_module!("fs";
    funcs: [
        mkdir, mkdir_all, rmdir, rmdir_all,
        list_dir,
        copy_file, move_file,
        file_size, file_modified,
        temp_dir,
        rename_file,
    ],
);
