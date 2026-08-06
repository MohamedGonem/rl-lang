use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vf, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_progress") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::ProgressBar(p)) => vok!(vf!(p.value as f64)),
        Some(_) => verr!(vs!(format!(
            "gui_get_progress: handle {} is not a progress bar",
            id
        ))),
        None => verr!(vs!(format!("gui_get_progress: unknown handle {}", id))),
    }
}
