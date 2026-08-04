use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::VmValue,
};
use rl_ast::statements::HandleKind;

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.net_handles.get(&id) {
        Some(NetHandle::TcpListener(_)) | Some(NetHandle::TcpStream(_)) => {
            eval.net_handles.remove(&id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "tcp_close(): handle {} is not a TCP handle",
            id
        ))),
        None => verr!(vs!(format!("tcp_close(): unknown handle {}", id))),
    }
}
