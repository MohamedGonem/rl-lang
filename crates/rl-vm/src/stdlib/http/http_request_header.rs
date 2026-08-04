use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};
pub fn func(eval: &mut Evaluator, handle: Value, name: String) -> Value {
    let id = match extract_handle(handle, HandleKind::Http, "http_request_header") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let request = match eval.http_handles.get(&id) {
        Some(HttpHandle::Request(request)) => request,
        Some(_) => {
            return verr!(vs!(format!(
                "http_request_header: handle {} is not a request",
                id
            )));
        }
        None => {
            return verr!(vs!(format!("http_request_header: unknown handle {}", id)));
        }
    };
    let found = request
        .headers()
        .iter()
        .find(|h| h.field.to_string().eq_ignore_ascii_case(&name))
        .map(|h| h.value.to_string());
    match found {
        Some(value) => vok!(vs!(value)),
        None => verr!(vs!(format!("http_request_header: no \"{}\" header", name))),
    }
}
