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

pub fn func(eval: &mut Vm, window: VmValue, title: String) -> VmValue {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_title") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.title = title;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_title: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_window_set_title: unknown handle {}", id))),
    }
}
