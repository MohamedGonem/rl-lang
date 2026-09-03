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

#[native_fn(module = "fs")]
pub fn list_dir_names(path: String) -> Result<Vec<String>, String> {
    match std::fs::read_dir(&path) {
        Err(e) => Err(format!(
            "list_dir_names: failed to read \"{}\": {}",
            path, e
        )),
        Ok(d) => Ok(d
            .filter_map(|i| i.ok())
            .map(|i| {
                i.file_name()
                    .to_string_lossy()
                    .to_string()
            })
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

#[native_fn(module = "fs")]
pub fn file_created(path: String) -> Result<i64, String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => return Err(format!("file_created: failed to read \"{}\": {}", path, e)),
    };
    match metadata.created() {
        Err(e) => Err(format!(
            "file_created: creation time not available for \"{}\": {}",
            path, e
        )),
        Ok(t) => match t.duration_since(std::time::UNIX_EPOCH) {
            Err(e) => Err(format!(
                "file_created: creation time before epoch for \"{}\": {}",
                path, e
            )),
            Ok(d) => Ok(d.as_secs() as i64),
        },
    }
}

#[native_fn(module = "fs")]
pub fn file_accessed(path: String) -> Result<i64, String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "file_accessed: failed to read \"{}\": {}",
                path, e
            ))
        }
    };
    match metadata.accessed() {
        Err(e) => Err(format!(
            "file_accessed: access time not available for \"{}\": {}",
            path, e
        )),
        Ok(t) => match t.duration_since(std::time::UNIX_EPOCH) {
            Err(e) => Err(format!(
                "file_accessed: access time before epoch for \"{}\": {}",
                path, e
            )),
            Ok(d) => Ok(d.as_secs() as i64),
        },
    }
}

// ---- permissions ----------------------------------------------------------

#[native_fn(module = "fs")]
pub fn file_permissions(path: String) -> Result<i64, String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "file_permissions: failed to read \"{}\": {}",
                path, e
            ))
        }
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        Ok(metadata.permissions().mode() as i64)
    }
    #[cfg(not(unix))]
    {
        Ok(if metadata.permissions().readonly() { 0o444 } else { 0o644 })
    }
}

#[native_fn(module = "fs")]
pub fn set_permissions(path: String, mode: i64) -> Result<(), String> {
    let metadata = match std::fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            return Err(format!(
                "set_permissions: failed to read \"{}\": {}",
                path, e
            ))
        }
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = metadata.permissions();
        perms.set_mode(mode as u32);
        match std::fs::set_permissions(&path, perms) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "set_permissions: failed to set permissions on \"{}\": {}",
                path, e
            )),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = (metadata, mode);
        Err("set_permissions: not supported on this platform".to_string())
    }
}

// ---- temp dir / file ------------------------------------------------------

#[native_fn(module = "fs")]
pub fn temp_dir() -> String {
    std::env::temp_dir().to_string_lossy().to_string()
}

#[native_fn(module = "fs")]
pub fn temp_file() -> Result<String, String> {
    match tempfile::NamedTempFile::new() {
        Ok(f) => Ok(f.into_temp_path().to_string_lossy().to_string()),
        Err(e) => Err(format!("temp_file: {}", e)),
    }
}

#[native_fn(module = "fs")]
pub fn temp_file_in(dir: String) -> Result<String, String> {
    match tempfile::Builder::new().tempfile_in(&dir) {
        Ok(f) => Ok(f.into_temp_path().to_string_lossy().to_string()),
        Err(e) => Err(format!("temp_file_in: {}", e)),
    }
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

// ---- touch / truncate -----------------------------------------------------

#[native_fn(module = "fs")]
pub fn touch(path: String) -> Result<(), String> {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("touch: failed to create \"{}\": {}", path, e)),
    }
}

#[native_fn(module = "fs")]
pub fn truncate_file(path: String, len: i64) -> Result<(), String> {
    match std::fs::File::open(&path) {
        Ok(f) => match f.set_len(len.max(0) as u64) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "truncate_file: failed to truncate \"{}\": {}",
                path, e
            )),
        },
        Err(e) => Err(format!(
            "truncate_file: failed to open \"{}\": {}",
            path, e
        )),
    }
}

// ---- glob -----------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn glob(pattern: String) -> Result<Vec<String>, String> {
    match ::glob::glob(&pattern) {
        Ok(paths) => Ok(paths
            .filter_map(|p| p.ok())
            .map(|p| p.to_string_lossy().to_string())
            .collect()),
        Err(e) => Err(format!("glob: invalid pattern \"{}\": {}", pattern, e)),
    }
}

// ---- walk -----------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn walk_dir(path: String) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let mut stack = vec![std::path::PathBuf::from(&path)];

    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                return Err(format!(
                    "walk_dir: failed to read \"{}\": {}",
                    dir.to_string_lossy(),
                    e
                ))
            }
        };

        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            results.push(path.to_string_lossy().to_string());
        }
    }

    Ok(results)
}

// ---- symlinks -------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn symlink(src: String, dst: String) -> Result<(), String> {
    #[cfg(unix)]
    {
        match std::os::unix::fs::symlink(&src, &dst) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "symlink: failed to create symlink from \"{}\" to \"{}\": {}",
                src, dst, e
            )),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = (src, dst);
        Err("symlink: not supported on this platform".to_string())
    }
}

#[native_fn(module = "fs")]
pub fn readlink(path: String) -> Result<String, String> {
    match std::fs::read_link(&path) {
        Ok(target) => Ok(target.to_string_lossy().to_string()),
        Err(e) => Err(format!(
            "readlink: failed to read symlink \"{}\": {}",
            path, e
        )),
    }
}

#[native_fn(module = "fs")]
pub fn hardlink(src: String, dst: String) -> Result<(), String> {
    match std::fs::hard_link(&src, &dst) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!(
            "hardlink: failed to create hard link from \"{}\" to \"{}\": {}",
            src, dst, e
        )),
    }
}

// ---- realpath -------------------------------------------------------------

#[native_fn(module = "fs")]
pub fn realpath(path: String) -> Result<String, String> {
    match std::fs::canonicalize(&path) {
        Ok(p) => Ok(p.to_string_lossy().to_string()),
        Err(e) => Err(format!("realpath: failed to resolve \"{}\": {}", path, e)),
    }
}

// ---- lock / unlock --------------------------------------------------------

#[native_fn(module = "fs")]
pub fn lock_file(path: String) -> Result<(), String> {
    use std::os::unix::io::AsRawFd;
    let file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
    {
        Ok(f) => f,
        Err(e) => return Err(format!("lock_file: failed to open \"{}\": {}", path, e)),
    };
    let ret = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) };
    if ret == 0 {
        std::mem::forget(file);
        Ok(())
    } else {
        Err(format!("lock_file: failed to lock \"{}\"", path))
    }
}

#[native_fn(module = "fs")]
pub fn unlock_file(path: String) -> Result<(), String> {
    use std::os::unix::io::AsRawFd;
    let file = match std::fs::OpenOptions::new()
        .write(true)
        .open(&path)
    {
        Ok(f) => f,
        Err(e) => {
            return Err(format!(
                "unlock_file: failed to open \"{}\": {}",
                path, e
            ))
        }
    };
    let ret = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
    if ret == 0 {
        Ok(())
    } else {
        Err(format!("unlock_file: failed to unlock \"{}\"", path))
    }
}

rl_std_core::native_module!("fs";
    funcs: [
        mkdir, mkdir_all, rmdir, rmdir_all,
        list_dir, list_dir_names,
        copy_file, move_file,
        file_size, file_modified, file_created, file_accessed,
        file_permissions, set_permissions,
        temp_dir, temp_file, temp_file_in,
        rename_file,
        touch, truncate_file,
        glob, walk_dir,
        symlink, readlink, hardlink,
        realpath,
        lock_file, unlock_file,
    ],
);
