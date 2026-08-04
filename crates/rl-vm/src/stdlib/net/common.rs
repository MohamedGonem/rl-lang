use crate::{evaluator::Evaluator, stdlib::net::NetHandle, values::Value};
use rl_ast::statements::HandleKind;

pub fn insert_handle(eval: &mut Evaluator, handle: NetHandle) -> Value {
    let id = eval.net_next_handle;
    eval.net_next_handle += 1;
    eval.net_handles.insert(id, handle);
    Value::Handle {
        kind: HandleKind::Net,
        id,
    }
}
