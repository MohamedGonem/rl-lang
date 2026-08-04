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

pub fn func(eval: &mut Vm, handle: VmValue, checked: bool) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_checked") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Checkbox(c)) => {
            c.checked = checked;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_set_checked: handle {} is not a checkbox",
            id
        ))),
        None => verr!(vs!(format!("gui_set_checked: unknown handle {}", id))),
    }
}
