use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, text: String) -> Value {
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
