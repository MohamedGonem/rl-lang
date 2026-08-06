use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_peer_addr") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_peer_addr: handle {} is not a TCP stream",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_peer_addr: unknown handle {}", id))),
    };
    match stream.peer_addr() {
        Ok(addr) => vok!(vs!(addr.to_string())),
        Err(e) => verr!(vs!(format!("tcp_peer_addr: {}", e))),
    }
}
