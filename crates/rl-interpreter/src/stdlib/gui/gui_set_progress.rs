use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, value: f64) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_progress") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::ProgressBar(p)) => {
            p.value = (value as f32).clamp(0.0, 1.0);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_set_progress: handle {} is not a progress bar",
            id
        ))),
        None => verr!(vs!(format!("gui_set_progress: unknown handle {}", id))),
    }
}
