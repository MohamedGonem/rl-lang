use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vi, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_selected_index") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            vok!(vi!(s.selected as i64))
        }
        Some(_) => verr!(vs!(format!(
            "gui_get_selected_index: handle {} has no selection",
            id
        ))),
        None => verr!(vs!(format!(
            "gui_get_selected_index: unknown handle {}",
            id
        ))),
    }
}
