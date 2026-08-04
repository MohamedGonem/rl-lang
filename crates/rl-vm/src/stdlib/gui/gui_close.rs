use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{GuiHandle, common::close_window},
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, window: VmValue) -> VmValue {
    let id = match extract_handle(window, HandleKind::Gui, "gui_close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Window(_)) => {
            close_window(eval, id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_close: handle {} is not a window", id))),
        None => verr!(vs!(format!("gui_close: unknown handle {}", id))),
    }
}
