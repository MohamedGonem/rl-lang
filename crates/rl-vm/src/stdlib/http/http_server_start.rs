use crate::{
    Vm,
    stdlib::{
        common::{verr, vok, vs},
        http::{HttpHandle, common::insert_handle},
    },
    values::VmValue,
};
pub fn func(eval: &mut Vm, addr: String) -> VmValue {
    match tiny_http::Server::http(&addr) {
        Ok(server) => {
            let id = insert_handle(eval, HttpHandle::Server(server));
            vok!(id)
        }
        Err(e) => verr!(vs!(format!("http_server_start(\"{}\"): {}", addr, e))),
    }
}
