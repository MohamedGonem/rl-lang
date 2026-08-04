use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{verr, vi, vok, vs},
        http::HttpHandle,
    },
    values::Value,
};

pub fn ureq_result_to_value(url: &str, result: Result<ureq::Response, ureq::Error>) -> Value {
    match result {
        Ok(response) => {
            let status = response.status() as i64;
            let body = response.into_string().unwrap_or_default();
            vok!(Value::Tuple(vec![vi!(status), vs!(body)]))
        }
        Err(ureq::Error::Status(code, response)) => {
            let body = response.into_string().unwrap_or_default();
            vok!(Value::Tuple(vec![vi!(code as i64), vs!(body)]))
        }
        Err(e) => {
            verr!(vs!(format!("{}: {}", url, e)))
        }
    }
}

pub fn insert_handle(eval: &mut Evaluator, handle: HttpHandle) -> Value {
    let id = eval.http_next_handle;
    eval.http_next_handle += 1;
    eval.http_handles.insert(id, handle);
    Value::Handle {
        kind: HandleKind::Http,
        id,
    }
}
