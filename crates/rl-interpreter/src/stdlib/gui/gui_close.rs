use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::{GuiHandle, common::close_window},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value) -> Value {
    let id = match extract_handle(window, HandleKind::Gui, "gui_close") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Window(_)) => {
            close_window(eval, id);
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_close: handle {} is not a window", id))),
        None => verr!(vs!(format!("gui_close: unknown handle {}", id))),
    }
}
