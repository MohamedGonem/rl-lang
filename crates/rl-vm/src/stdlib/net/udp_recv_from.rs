use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, extract_number, verr, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, max_bytes: Value) -> Value {
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
            vok!(Value::Tuple(vec![vs!(data), vs!(sender.to_string())]))
        }
        Err(e) => verr!(vs!(format!("udp_recv_from: {}", e))),
    }
}
