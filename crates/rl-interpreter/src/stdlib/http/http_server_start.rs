use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{verr, vok, vs},
        http::{HttpHandle, common::insert_handle},
    },
    values::Value,
};
pub fn func(eval: &mut Evaluator, addr: String) -> Value {
    match tiny_http::Server::http(&addr) {
        Ok(server) => {
            let id = insert_handle(eval, HttpHandle::Server(server));
            vok!(id)
        }
        Err(e) => verr!(vs!(format!("http_server_start(\"{}\"): {}", addr, e))),
    }
}
