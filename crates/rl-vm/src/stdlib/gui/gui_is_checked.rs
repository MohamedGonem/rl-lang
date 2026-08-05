use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{vb, verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_is_checked") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Checkbox(c)) => vok!(vb!(c.checked)),
        Some(_) => verr!(vs!(format!(
            "gui_is_checked: handle {} is not a checkbox",
            id
        ))),
        None => verr!(vs!(format!("gui_is_checked: unknown handle {}", id))),
    }
}
