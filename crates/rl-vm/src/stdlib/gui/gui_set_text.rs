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

pub fn func(eval: &mut Vm, handle: VmValue, text: String) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_text") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Button(b)) => {
            b.label = text;
            vok!(vnl!())
        }
        Some(GuiHandle::Label(l)) => {
            l.text = text;
            vok!(vnl!())
        }
        Some(GuiHandle::Textbox(t)) => {
            t.text = text;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_set_text: handle {} has no text", id))),
        None => verr!(vs!(format!("gui_set_text: unknown handle {}", id))),
    }
}
