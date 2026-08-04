use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "udp_close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(_)) => {
            eval.net_handles.remove(&id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("udp_close: handle {} is not a UDP socket", id))),
        None => verr!(vs!(format!("udp_close: unknown handle {}", id))),
    }
}
