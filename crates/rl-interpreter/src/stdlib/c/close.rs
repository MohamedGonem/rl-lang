use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_number, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let handle_id = match extract_number(handle, "close") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("close: {}", e))),
    };

    match eval.c_handles.remove(&handle_id) {
        Some(_) => vok!(vnl!()),
        None => verr!(vs!(format!("close: unknown handle {}", handle_id))),
    }
}
