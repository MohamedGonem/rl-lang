use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        http::HttpHandle,
    },
    values::VmValue,
};
pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Http, "http_server_stop") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.http_handles.get(&id) {
        Some(HttpHandle::Server(_)) => {
            eval.http_handles.remove(&id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "http_server_stop: handle {} is not a server",
            id
        ))),
        None => verr!(vs!(format!("http_server_stop: unknown handle {}", id))),
    }
}
