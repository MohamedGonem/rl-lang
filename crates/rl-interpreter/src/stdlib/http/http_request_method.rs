use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};
pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Http, "http_request_method") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.http_handles.get(&id) {
        Some(HttpHandle::Request(request)) => vok!(vs!(request.method().to_string())),
        Some(_) => verr!(vs!(format!(
            "http_request_method: handle {} is not a request",
            id
        ))),
        None => verr!(vs!(format!("http_request_method: unknown handle {}", id))),
    }
}
