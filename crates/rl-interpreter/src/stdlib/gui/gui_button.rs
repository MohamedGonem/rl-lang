use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{ButtonState, GuiHandle, common::insert_handle},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, label: String, x: i64, y: i64) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_button") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&window_id) {
        Some(GuiHandle::Window(_)) => {}
        Some(_) => {
            return verr!(vs!(format!(
                "gui_button: handle {} is not a window",
                window_id
            )));
        }
        None => return verr!(vs!(format!("gui_button: unknown handle {}", window_id))),
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Button(ButtonState {
            window: window_id,
            label,
            x: x as f32,
            y: y as f32,
            visible: true,
            on_click: None,
        }),
    );

    let Value::Handle { id: button_id, .. } = handle else {
        unreachable!()
    };

    if let Some(GuiHandle::Window(w)) = eval.gui_handles.get_mut(&window_id) {
        w.children.push(button_id);
    }

    vok!(handle)
}
