use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        c::CHandle,
        common::{extract_handle, extract_string, vb, verr, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, fn_name: Value) -> Value {
    let handle_id = match extract_handle(handle, HandleKind::C, "has_symbol") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let fn_name = match extract_string(fn_name, "has_symbol") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("has_symbol: {}", e))),
    };

    let CHandle::Library(lib) = match eval.c_handles.get(&handle_id) {
        Some(h) => h,
        None => return verr!(vs!(format!("has_symbol: unknown handle {}", handle_id))),
    };

    // SAFETY: same as `call`'s symbol lookup - resolving a symbol address is
    // unsafe per `libloading`'s contract, but we never call through it here,
    // only check whether the lookup itself succeeds.
    let found = unsafe { lib.get::<*const ()>(fn_name.as_bytes()) }.is_ok();
    vok!(vb!(found))
}
