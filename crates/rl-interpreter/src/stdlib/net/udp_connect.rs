use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, extract_string, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, address: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Net, "udp_connect") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let addr = match extract_string(address, "udp_connect") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("udp_connect: {} ", e))),
    };

    let socket = match eval.net_handles.get(&id) {
        Some(NetHandle::UdpSocket(socket)) => socket,
        Some(_) => {
            return verr!(vs!(format!(
                "udp_connect: handle {} is not a UDP socket",
                id
            )));
        }
        None => return verr!(vs!(format!("udp_connect: unknown handle {}", id))),
    };
    match socket.connect(&addr) {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("udp_connect(\"{}\"): {}", addr, e))),
    }
}
