use std::io::Read;

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
    let id = match extract_handle(handle, HandleKind::Net, "tcp_read") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let max_bytes = match extract_number(max_bytes, "tcp_read") {
        Ok(a) => a as usize,
        Err(e) => return verr!(vs!(format!("tcp_read: {}", e))),
    };

    let stream = match eval.net_handles.get_mut(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!("tcp_read: handle {} is not a TCP stream", id)));
        }
        None => return verr!(vs!(format!("tcp_read: unknown handle {}", id))),
    };

    let mut buf = vec![0u8; max_bytes];
    match stream.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            vok!(vs!(String::from_utf8_lossy(&buf).into_owned()))
        }
        Err(e) => verr!(vs!(format!("tcp_read: {}", e))),
    }
}
