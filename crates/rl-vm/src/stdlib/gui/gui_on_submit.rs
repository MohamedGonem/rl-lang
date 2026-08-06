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

pub fn func(eval: &mut Vm, handle: VmValue, function: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_on_submit") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if !matches!(function, VmValue::Function { .. }) {
        return verr!(vs!(format!(
            "gui_on_submit: expected function or lambda, found {}",
            function.type_name()
        )));
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Textbox(t)) if t.multiline => verr!(vs!(format!(
            "gui_on_submit: handle {} is a multiline textarea - Enter inserts a newline there instead of submitting, so on_submit never fires. Use a button instead",
            id
        ))),
        Some(GuiHandle::Textbox(t)) => {
            t.on_submit = Some(function);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_on_submit: handle {} is not a textbox",
            id
        ))),
        None => verr!(vs!(format!("gui_on_submit: unknown handle {}", id))),
    }
}
