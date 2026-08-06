use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, width: i64, height: i64) -> Value {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_size") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_size = Some((width.max(1) as f32, height.max(1) as f32));
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_size: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_window_set_size: unknown handle {}", id))),
    }
}
