use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        c::CHandle,
        common::{extract_handle, extract_string},
        macros::{vb, verr, vok, vs},
    },
    values::VmValue,
};

pub fn std_has_symbol(vm: &mut Vm, handle: VmValue, fn_name: VmValue) -> VmValue {
    let handle_id = match extract_handle(handle, HandleKind::C, "has_symbol") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("has_symbol: {}", e))),
    };
    let fn_name = match extract_string(fn_name, "has_symbol") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("has_symbol: {}", e))),
    };

    let CHandle::Library(lib) = match vm.c_handles.get(&handle_id) {
        Some(h) => h,
        None => return verr!(vs!(format!("has_symbol: unknown handle {}", handle_id))),
    };

    // SAFETY: same as `call`'s symbol lookup - resolving a symbol address is
    // unsafe per `libloading`'s contract, but we never call through it here,
    // only check whether the lookup itself succeeds.
    let found = unsafe { lib.get::<*const ()>(fn_name.as_bytes()) }.is_ok();
    vok!(vb!(found))
}
