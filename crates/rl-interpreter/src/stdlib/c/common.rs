use crate::evaluator::Evaluator;
use crate::stdlib::c::CHandle;

/// Registers a new handle and returns its id.
pub fn insert_handle(eval: &mut Evaluator, handle: CHandle) -> i64 {
    let id = eval.c_next_handle;
    eval.c_next_handle += 1;
    eval.c_handles.insert(id, handle);
    id
}
