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
    let id = match extract_handle(handle, HandleKind::Gui, "gui_on_click") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if !matches!(function, VmValue::Function { .. }) {
        return verr!(vs!(format!(
            "gui_on_click: expected function or lambda, found {}",
            function.type_name()
        )));
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Button(b)) => {
            b.on_click = Some(function);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_on_click: handle {} is not a button", id))),
        None => verr!(vs!(format!("gui_on_click: unknown handle {}", id))),
    }
}
