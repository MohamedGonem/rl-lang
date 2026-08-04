use crate::{Vm, stdlib::net::NetHandle, values::VmValue};
use rl_ast::statements::HandleKind;

pub fn insert_handle(eval: &mut Vm, handle: NetHandle) -> VmValue {
    let id = eval.net_next_handle;
    eval.net_next_handle += 1;
    eval.net_handles.insert(id, handle);
    VmValue::Handle {
        kind: HandleKind::Net,
        id,
    }
}
