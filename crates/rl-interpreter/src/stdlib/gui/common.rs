use crate::{evaluator::Evaluator, stdlib::gui::GuiHandle, values::Value};
use rl_ast::statements::HandleKind;

pub fn insert_handle(eval: &mut Evaluator, handle: GuiHandle) -> Value {
    let id = eval.gui_next_handle;
    eval.gui_next_handle += 1;
    eval.gui_handles.insert(id, handle);
    Value::Handle {
        kind: HandleKind::Gui,
        id,
    }
}

pub fn require_window(eval: &Evaluator, window_id: u64, fn_name: &str) -> Result<(), String> {
    match eval.gui_handles.get(&window_id) {
        Some(GuiHandle::Window(_)) => Ok(()),
        Some(_) => Err(format!("{}: handle {} is not a window", fn_name, window_id)),
        None => Err(format!("{}: unknown handle {}", fn_name, window_id)),
    }
}

pub fn attach_child(eval: &mut Evaluator, window_id: u64, child_id: u64) {
    if let Some(GuiHandle::Window(w)) = eval.gui_handles.get_mut(&window_id) {
        w.children.push(child_id);
    }
}

