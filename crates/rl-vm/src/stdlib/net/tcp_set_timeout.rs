use std::time::Duration;

use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, extract_number, verr, vnl, vok, vs},
        net::NetHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, millis: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Net, "tcp_set_timeout") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let millis = match extract_number(millis, "tcp_set_timeout") {
        Ok(a) => a,
        Err(e) => return verr!(vs!(format!("tcp_set_timeout: {}", e))),
    };

    let stream = match eval.net_handles.get(&id) {
        Some(NetHandle::TcpStream(stream)) => stream,
        Some(_) => {
            return verr!(vs!(format!(
                "tcp_set_timeout: handle {} is not a TCP stream",
                id
            )));
        }
        None => return verr!(vs!(format!("tcp_set_timeout: unknown handle {}", id))),
    };
    let duration = if millis == 0 {
        None
    } else {
        Some(Duration::from_millis(millis))
    };
    let result = stream
        .set_read_timeout(duration)
        .and_then(|_| stream.set_write_timeout(duration));
    match result {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("tcp_set_timeout: {}", e))),
    }
}
