use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, checked: bool) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_checked") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Checkbox(c)) => {
            c.checked = checked;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_set_checked: handle {} is not a checkbox",
            id
        ))),
        None => verr!(vs!(format!("gui_set_checked: unknown handle {}", id))),
    }
}
