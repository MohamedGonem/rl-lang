use std::net::Shutdown;

use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, extract_string, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, mode: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_local_addr") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let mode = match extract_string(mode, "tcp_shutdown") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("tcp_shutdown: {} ", e))),
    };

    let mode = match mode.as_str() {
        "read" => Shutdown::Read,
        "write" => Shutdown::Write,
        "both" => Shutdown::Both,
        other => {
            return verr!(vs!(format!(
                "tcp_shutdown: expected \"read\", \"write\", or \"both\", got \"{}\"",
                other
            )));
        }
    };
    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_shutdown: handle {} is not a TCP stream",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_shutdown: unknown handle {}", id))),
    };
    match stream.shutdown(mode) {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("tcp_shutdown(): {}", e))),
    }
}
