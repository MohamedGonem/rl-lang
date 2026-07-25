use crate::{
    Vm,
    stdlib::{
        common::extract_number,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn std_close(vm: &mut Vm, handle: VmValue) -> VmValue {
    let handle_id = match extract_number(handle, "close") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("close: {}", e))),
    };

    match vm.c_handles.remove(&handle_id) {
        Some(_) => vok!(vnl!()),
        None => verr!(vs!(format!("close: unknown handle {}", handle_id))),
    }
}
