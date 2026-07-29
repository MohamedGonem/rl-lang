use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_handle, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let handle_id = match extract_handle(handle, HandleKind::C, "close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.c_handles.remove(&handle_id) {
        Some(_) => vok!(vnl!()),
        None => verr!(vs!(format!("close: unknown handle {}", handle_id))),
    }
}
