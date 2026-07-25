use crate::Vm;
use crate::stdlib::c::CHandle;

/// Registers a new handle and returns its id.
pub fn insert_handle(vm: &mut Vm, handle: CHandle) -> i64 {
    let id = vm.c_next_handle;
    vm.c_next_handle += 1;
    vm.c_handles.insert(id, handle);
    id
}
