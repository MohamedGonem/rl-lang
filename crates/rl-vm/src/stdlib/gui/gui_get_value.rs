use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{verr, vf, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_value") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Slider(s)) => vok!(vf!(s.value)),
        Some(_) => verr!(vs!(format!("gui_get_value: handle {} is not a slider", id))),
        None => verr!(vs!(format!("gui_get_value: unknown handle {}", id))),
    }
}
