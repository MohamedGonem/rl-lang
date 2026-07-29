use std::io::Write;

use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, extract_string, verr, vi, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, data: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_local_addr") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let data = match extract_string(data, "tcp_write") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("tcp_write: {} ", e))),
    };

    let stream = match eval.net_handles.get_mut(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!("tcp_write: handle {} is not a TCP stream", id)));
        }
        None => return verr!(vs!(format!("tcp_write: unknown handle {}", id))),
    };

    match stream.write(data.as_bytes()) {
        Ok(n) => vok!(vi!(n as i64)),
        Err(e) => verr!(vs!(format!("tcp_write: {}", e))),
    }
}
