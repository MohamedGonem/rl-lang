use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, vb, verr, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_is_checked") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Checkbox(c)) => vok!(vb!(c.checked)),
        Some(_) => verr!(vs!(format!(
            "gui_is_checked: handle {} is not a checkbox",
            id
        ))),
        None => verr!(vs!(format!("gui_is_checked: unknown handle {}", id))),
    }
}
