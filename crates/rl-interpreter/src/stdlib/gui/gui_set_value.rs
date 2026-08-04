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
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_value") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Slider(s)) => {
            s.value = value.clamp(s.min, s.max);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_set_value: handle {} is not a slider", id))),
        None => verr!(vs!(format!("gui_set_value: unknown handle {}", id))),
    }
}
