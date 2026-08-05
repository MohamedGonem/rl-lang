use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, extract_number, verr, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue, max_bytes: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "udp_recv") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let max_bytes = match extract_number(max_bytes, "udp_recv") {
        Ok(a) => a as usize,
        Err(e) => return verr!(vs!(format!("udp_recv: {}", e))),
    };
    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!("udp_recv: handle {} is not a UDP socket", id)));
        }
        None => return verr!(vs!(format!("udp_recv: unknown handle {}", id))),
    };
    let mut buf = vec![0u8; max_bytes];
    match socket.recv(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            vok!(vs!(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => verr!(vs!(format!("udp_recv: {}", e))),
    }
}
