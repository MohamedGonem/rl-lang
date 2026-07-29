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
    let id = match extract_handle(handle, HandleKind::Http, "http_request_body") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let request = match eval.http_handles.get_mut(&id) {
        Some(HttpHandle::Request(request)) => request,
        Some(_) => {
            return verr!(vs!(format!(
                "http_request_body(): handle {} is not a request",
                id
            )));
        }
        None => return verr!(vs!(format!("http_request_body(): unknown handle {}", id))),
    };

    let mut body = String::new();
    match request.as_reader().read_to_string(&mut body) {
        Ok(_) => vok!(vs!(body)),
        Err(e) => verr!(vs!(format!("http_request_body(): {}", e))),
    }
}
