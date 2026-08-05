use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_text") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Button(b)) => vok!(vs!(b.label.clone())),
        Some(GuiHandle::Label(l)) => vok!(vs!(l.text.clone())),
        Some(GuiHandle::Textbox(t)) => vok!(vs!(t.text.clone())),
        Some(_) => verr!(vs!(format!("gui_get_text: handle {} has no text", id))),
        None => verr!(vs!(format!("gui_get_text: unknown handle {}", id))),
    }
}
