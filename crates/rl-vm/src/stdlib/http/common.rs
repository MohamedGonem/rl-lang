use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::{verr, vi, vok, vs},
        http::HttpHandle,
    },
    values::VmValue,
};
use std::rc::Rc;

pub fn ureq_result_to_value(url: &str, result: Result<ureq::Response, ureq::Error>) -> VmValue {
    match result {
        Ok(response) => {
            let status = response.status() as i64;
            let body = response.into_string().unwrap_or_default();
            vok!(VmValue::Tuple(Rc::new(vec![vi!(status), vs!(body)])))
        }
        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_default();
            vok!(VmValue::Tuple(Rc::new(vec![vi!(code as i64), vs!(body)])))
        }
        Err(e) => {
            verr!(vs!(format!("{}: {}", url, e)))
        }
    }
}

pub fn insert_handle(eval: &mut Vm, handle: HttpHandle) -> VmValue {
    let id = eval.http_next_handle;
    eval.http_next_handle += 1;
    eval.http_handles.insert(id, handle);
    VmValue::Handle {
        kind: HandleKind::Http,
        id,
    }
}
