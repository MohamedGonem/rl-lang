use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, r: i64, g: i64, b: i64) -> Value {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_background") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    for (name, channel) in [("r", r), ("g", g), ("b", b)] {
        if !(0..=255).contains(&channel) {
            return verr!(vs!(format!(
                "gui_window_set_background: {} ({}) must be between 0 and 255",
                name, channel
            )));
        }
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.background = (r as u8, g as u8, b as u8);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_background: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!(
            "gui_window_set_background: unknown handle {}",
            id
        ))),
    }
}
