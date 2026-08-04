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

pub fn func(eval: &mut Vm, window: VmValue, decorated: bool) -> VmValue {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_decorated") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.decorated = decorated;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_decorated: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!(
            "gui_window_set_decorated: unknown handle {}",
            id
        ))),
    }
}
