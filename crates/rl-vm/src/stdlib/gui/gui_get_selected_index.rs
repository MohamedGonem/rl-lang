use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{verr, vi, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
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
