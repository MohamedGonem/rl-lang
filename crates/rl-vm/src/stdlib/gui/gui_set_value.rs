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

pub fn func(eval: &mut Vm, handle: VmValue, value: f64) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_value") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Slider(s)) => {
            s.value = value.clamp(s.min, s.max);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_set_value: handle {} is not a slider", id))),
        None => verr!(vs!(format!("gui_set_value: unknown handle {}", id))),
    }
}
