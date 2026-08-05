use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, x: i64, y: i64) -> Value {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_pos") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.pending_position = Some((x as f32, y as f32));
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_pos: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_window_set_pos: unknown handle {}", id))),
    }
}
