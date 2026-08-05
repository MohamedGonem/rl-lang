use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        net::{NetHandle, common::insert_handle},
    },
    values::VmValue,
};
use rl_ast::statements::HandleKind;

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_accept") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let accept_result = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpListener(listener)) => listener.accept(),
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_accept: handle {} is not a TCP listener",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_accept: unknown handle {}", id))),
    };

    match accept_result {
        Ok((stream, _addr)) => {
            let handle = insert_handle(eval, NetHandle::TcpStream(stream));
            vok!(handle)
        }
        Err(e) => verr!(vs!(format!("tcp_accept(): {}", e))),
    }
}
