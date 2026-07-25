use std::path::PathBuf;

use crate::evaluator::Evaluator;
use crate::stdlib::c::CHandle;

/// Registers a new handle and returns its id.
pub fn insert_handle(eval: &mut Evaluator, handle: CHandle) -> i64 {
    let id = eval.c_next_handle;
    eval.c_next_handle += 1;
    eval.c_handles.insert(id, handle);
    id
}

/// Where `compile` caches built shared libraries by content hash, and what
/// `clear_cache` empties. Kept here (not duplicated in `compile.rs`) so
/// there's exactly one place both agree on the path.
pub fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("rl_std_c_cache")
}
