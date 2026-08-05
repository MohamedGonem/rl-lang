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
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_z") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let z = match eval.gui_handles.get(&id) {
        Some(GuiHandle::Button(w)) => w.z,
        Some(GuiHandle::Label(w)) => w.z,
        Some(GuiHandle::Checkbox(w)) => w.z,
        Some(GuiHandle::Textbox(w)) => w.z,
        Some(GuiHandle::Dropdown(w)) => w.z,
        Some(GuiHandle::RadioGroup(w)) => w.z,
        Some(GuiHandle::Slider(w)) => w.z,
        Some(GuiHandle::ProgressBar(w)) => w.z,
        Some(GuiHandle::Separator(w)) => w.z,
        Some(GuiHandle::Image(w)) => w.z,
        Some(GuiHandle::Window(_)) => {
            return verr!(vs!(format!("gui_get_z: handle {} is a window", id)));
        }
        None => return verr!(vs!(format!("gui_get_z: unknown handle {}", id))),
    };

    vok!(vi!(z as i64))
}
