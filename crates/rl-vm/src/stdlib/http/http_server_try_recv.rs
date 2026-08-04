use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        http::{HttpHandle, common::insert_handle},
    },
    values::VmValue,
};
pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Http, "http_server_try_recv") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let server = match eval.http_handles.get(&id) {
        Some(HttpHandle::Server(server)) => server,
        Some(_) => {
            return verr!(vs!(format!(
                "http_server_try_recv(): handle {} is not a server",
                id
            )));
        }
        None => {
            return verr!(vs!(format!("http_server_try_recv: unknown handle {}", id)));
        }
    };
    match server.try_recv() {
        Ok(Some(request)) => {
            let req_id = insert_handle(eval, HttpHandle::Request(request));
            vok!(req_id)
        }
        Ok(None) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("http_server_try_recv: {}", e))),
    }
}
