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

