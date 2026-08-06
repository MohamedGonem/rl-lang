use std::path::PathBuf;

use rl_ast::statements::HandleKind;

use crate::Vm;
use crate::stdlib::c::CHandle;
use crate::values::VmValue;

/// Registers a new handle and returns its id.
pub fn insert_handle(vm: &mut Vm, handle: CHandle) -> VmValue {
    let id = vm.c_next_handle;
    vm.c_next_handle += 1;
    vm.c_handles.insert(id, handle);
    VmValue::Handle {
        kind: HandleKind::C,
        id,
    }
}

/// Where `compile` caches built shared libraries by content hash, and what
/// `clear_cache` empties. Kept here (not duplicated in `compile.rs`) so
/// there's exactly one place both agree on the path - and deliberately the
/// *same* path `rl-interpreter`'s `std::c::compile` uses, so switching
/// between interpreter and VM doesn't force a recompile.
pub fn cache_dir() -> PathBuf {
    std::env::temp_dir().join("rl_std_c_cache")
}
