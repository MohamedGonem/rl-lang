use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, decorated: bool) -> Value {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_decorated") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.decorated = decorated;
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_decorated: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!(
            "gui_window_set_decorated: unknown handle {}",
            id
        ))),
    }
}
