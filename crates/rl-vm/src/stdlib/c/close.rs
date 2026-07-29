use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn std_close(vm: &mut Vm, handle: VmValue) -> VmValue {
    let handle_id = match extract_handle(handle, HandleKind::C, "close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("close: {}", e))),
    };

    match vm.c_handles.remove(&handle_id) {
        Some(_) => vok!(vnl!()),
        None => verr!(vs!(format!("close: unknown handle {}", handle_id))),
    }
}
