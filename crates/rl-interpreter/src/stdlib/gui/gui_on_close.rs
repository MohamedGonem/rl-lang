use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, function: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_on_close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if !matches!(function, Value::Function { .. }) {
        return verr!(vs!(format!(
            "gui_on_close: expected function or lambda, found {}",
            function.type_name()
        )));
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.on_close = Some(function);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_on_close: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_on_close: unknown handle {}", id))),
    }
}
