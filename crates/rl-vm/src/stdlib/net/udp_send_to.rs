use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, extract_string, verr, vi, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue, data: VmValue, address: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "udp_send_to") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let data = match extract_string(data, "udp_send_to") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let addr = match extract_string(address, "udp_send_to") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!(
                "udp_send_to: handle {} is not a UDP socket",
                id
            )));
        }
        None => return verr!(vs!(format!("udp_send_to: unknown handle {}", id))),
    };
    match socket.send_to(data.as_bytes(), &addr) {
        Ok(n) => vok!(vi!(n as i64)),
        Err(e) => verr!(vs!(format!("udp_send_to(\"{}\"): {}", addr, e))),
    }
}
