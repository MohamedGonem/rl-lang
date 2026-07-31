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
    let id = match extract_handle(handle, HandleKind::Gui, "gui_on_click") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if !matches!(function, Value::Function { .. }) {
        return verr!(vs!(format!(
            "gui_on_click: expected function or lambda, found {}",
            function.type_name()
        )));
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Button(b)) => {
            b.on_click = Some(function);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_on_click: handle {} is not a button", id))),
        None => verr!(vs!(format!("gui_on_click: unknown handle {}", id))),
    }
}
