use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, extract_number, verr, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};
use std::rc::Rc;

pub fn func(eval: &mut Vm, handle: VmValue, max_bytes: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "udp_recv_from") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let max_bytes = match extract_number(max_bytes, "udp_recv_from") {
        Ok(a) => a as usize,
        Err(e) => return verr!(vs!(format!("{}", e))),
    };
    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!(
                "udp_recv_from: handle {} is not a UDP socket",
                id
            )));
        }
        None => return verr!(vs!(format!("udp_recv_from: unknown handle {}", id))),
    };
    let mut buf = vec![0u8; max_bytes];
    match socket.recv_from(&mut buf) {
        Ok((n, sender)) => {
            buf.truncate(n);
            let data = String::from_utf8_lossy(&buf).into_owned();
            vok!(VmValue::Tuple(Rc::new(vec![
                vs!(data),
                vs!(sender.to_string())
            ])))
        }
        Err(e) => verr!(vs!(format!("udp_recv_from: {}", e))),
    }
}
