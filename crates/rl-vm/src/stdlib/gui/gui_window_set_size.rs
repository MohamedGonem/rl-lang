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

pub fn func(eval: &mut Vm, window: VmValue, width: i64, height: i64) -> VmValue {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_size") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_size = Some((width.max(1) as f32, height.max(1) as f32));
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_size: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_window_set_size: unknown handle {}", id))),
    }
}
