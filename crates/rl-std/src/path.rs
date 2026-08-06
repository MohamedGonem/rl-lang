//! `std::path` - path manipulation using [`std::path::PathBuf`].
//!
//! All functions take path strings and return strings or booleans.
//! `path_pop` removes the last component (returns the shortened path).
//! `path_push` appends a component (same as `path_join`).
//!
//! Ported once from the former per-runtime `stdlib/path/*.rs` copies. The
//! four "component" queries (`path_extension`/`path_filename`/`path_parent`/
//! `path_stem`) return either a string or `null`, so they build a raw
//! `R::Value` and carry an explicit `(string) -> string` signature.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;

// ---- predicates: `(string) -> bool` ---------------------------------------

#[native_fn(module = "path")]
pub fn path_exists(path: String) -> bool {
    std::path::Path::new(&path).exists()
}

#[native_fn(module = "path")]
pub fn path_is_dir(path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}

#[native_fn(module = "path")]
pub fn path_is_file(path: String) -> bool {
    std::path::Path::new(&path).is_file()
}

// ---- component queries: `(string) -> string` (or `null`) ------------------

#[native_fn(module = "path", sig(string -> string))]
pub fn path_extension<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).extension() {
        Some(ext) => R::from_string(ext.to_string_lossy().to_string()),
        None => R::null(),
    }
}

#[native_fn(module = "path", sig(string -> string))]
pub fn path_filename<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).file_name() {
        Some(name) => R::from_string(name.to_string_lossy().to_string()),
        None => R::null(),
    }
}

#[native_fn(module = "path", sig(string -> string))]
pub fn path_parent<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).parent() {
        Some(p) => R::from_string(p.to_string_lossy().to_string()),
        None => R::null(),
    }
}

#[native_fn(module = "path", sig(string -> string))]
pub fn path_stem<R: Runtime>(path: String) -> R::Value {
    match std::path::Path::new(&path).file_stem() {
        Some(stem) => R::from_string(stem.to_string_lossy().to_string()),
        None => R::null(),
    }
}

// ---- `(string) -> string` (always) ----------------------------------------

#[native_fn(module = "path")]
pub fn path_pop(path: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.pop();
    buf.to_string_lossy().to_string()
}

// ---- `(string, string) -> string` -----------------------------------------

#[native_fn(module = "path")]
pub fn path_join(path: String, target: String) -> String {
    std::path::PathBuf::from(&path)
        .join(&target)
        .to_string_lossy()
        .to_string()
}

#[native_fn(module = "path")]
pub fn path_push(path: String, target: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.push(&target);
    buf.to_string_lossy().to_string()
}

#[native_fn(module = "path")]
pub fn path_set_extension(path: String, target: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.set_extension(&target);
    buf.to_string_lossy().to_string()
}

rl_std_core::native_module!("path";
    funcs: [
        path_exists, path_is_dir, path_is_file,
        path_extension, path_filename, path_parent, path_stem,
        path_pop,
        path_join, path_push, path_set_extension,
    ],
);
