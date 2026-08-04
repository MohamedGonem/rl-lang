use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        http::HttpHandle,
    },
    values::VmValue,
};
pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Http, "http_request_url") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.http_handles.get(&id) {
        Some(HttpHandle::Request(request)) => vok!(vs!(request.url().to_string())),
        Some(_) => verr!(vs!(format!(
            "http_request_url: handle {} is not a request",
            id
        ))),
        None => verr!(vs!(format!("http_request_url: unknown handle {}", id))),
    }
}
