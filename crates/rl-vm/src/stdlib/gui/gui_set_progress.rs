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
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_progress") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::ProgressBar(p)) => {
            p.value = (value as f32).clamp(0.0, 1.0);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_set_progress: handle {} is not a progress bar",
            id
        ))),
        None => verr!(vs!(format!("gui_set_progress: unknown handle {}", id))),
    }
}
