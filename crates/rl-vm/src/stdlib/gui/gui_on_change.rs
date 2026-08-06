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
    let id = match extract_handle(handle, HandleKind::Gui, "gui_on_change") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if !matches!(function, VmValue::Function { .. }) {
        return verr!(vs!(format!(
            "gui_on_change: expected function or lambda, found {}",
            function.type_name()
        )));
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Checkbox(c)) => {
            c.on_change = Some(function);
            vok!(vnl!())
        }
        Some(GuiHandle::Textbox(t)) => {
            t.on_change = Some(function);
            vok!(vnl!())
        }
        Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
            s.on_change = Some(function);
            vok!(vnl!())
        }
        Some(GuiHandle::Slider(s)) => {
            s.on_change = Some(function);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_on_change: handle {} does not support change events",
            id
        ))),
        None => verr!(vs!(format!("gui_on_change: unknown handle {}", id))),
    }
}
