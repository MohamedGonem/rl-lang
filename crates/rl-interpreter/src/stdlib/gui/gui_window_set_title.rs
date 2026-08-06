use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, title: String) -> Value {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_title") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.title = title;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_title: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_window_set_title: unknown handle {}", id))),
    }
}
