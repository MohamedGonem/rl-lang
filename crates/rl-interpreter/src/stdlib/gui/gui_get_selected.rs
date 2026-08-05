use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_selected") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            match s.options.get(s.selected) {
                Some(opt) => vok!(vs!(opt.clone())),
                None => verr!(vs!(format!(
                    "gui_get_selected: handle {} has no options",
                    id
                ))),
            }
        }
        Some(_) => verr!(vs!(format!(
            "gui_get_selected: handle {} has no selection",
            id
        ))),
        None => verr!(vs!(format!("gui_get_selected: unknown handle {}", id))),
    }
}
