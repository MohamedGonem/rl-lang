use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue, index: i64) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_selected_index") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            if index < 0 || index as usize >= s.options.len() {
                return verr!(vs!(format!(
                    "gui_set_selected_index: index {} out of range (0..{})",
                    index,
                    s.options.len()
                )));
            }
            s.selected = index as usize;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_set_selected_index: handle {} has no selection",
            id
        ))),
        None => verr!(vs!(format!(
            "gui_set_selected_index: unknown handle {}",
            id
        ))),
    }
}
