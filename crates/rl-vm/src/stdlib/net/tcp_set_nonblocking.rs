use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue, flag: bool) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_set_nonblocking") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_set_nonblocking: handle {} is not a TCP stream",
                id
            )));
        }
        None => {
            return verr!(vs!(format!("tcp_set_nonblocking: unknown handle {}", id)));
        }
    };

    match stream.set_nonblocking(flag) {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("tcp_set_nonblocking: {}", e))),
    }
}
