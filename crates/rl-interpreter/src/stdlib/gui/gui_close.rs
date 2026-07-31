use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
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
            let children = match eval.gui_handles.remove(&id) {
                Some(GuiHandle::Window(w)) => w.children,
                _ => unreachable!(),
            };
            for child in children {
                eval.gui_handles.remove(&child);
            }
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!("gui_close: handle {} is not a window", id))),
        None => verr!(vs!(format!("gui_close: unknown handle {}", id))),
    }
}
