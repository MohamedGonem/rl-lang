use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{GuiHandle, LabelState, common::insert_handle},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, text: String, x: i64, y: i64) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_label") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&window_id) {
        Some(GuiHandle::Window(_)) => {}
        Some(_) => {
            return verr!(vs!(format!(
                "gui_label: handle {} is not a window",
                window_id
            )));
        }
        None => return verr!(vs!(format!("gui_label: unknown handle {}", window_id))),
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Label(LabelState {
            window: window_id,
            text,
            x: x as f32,
            y: y as f32,
            visible: true,
        }),
    );

    let Value::Handle { id: label_id, .. } = handle else {
        unreachable!()
    };

    if let Some(GuiHandle::Window(w)) = eval.gui_handles.get_mut(&window_id) {
        w.children.push(label_id);
    }

    vok!(handle)
}
