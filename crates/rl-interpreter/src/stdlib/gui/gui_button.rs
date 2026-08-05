use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            ButtonState, GuiHandle,
            common::{attach_child, insert_handle, require_window},
        },
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, label: String, x: i64, y: i64) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_button") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if let Err(e) = require_window(eval, window_id, "gui_button") {
        return verr!(vs!(e));
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
            z: 0,
        }),
    );

    let Value::Handle { id: button_id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, button_id);

    vok!(handle)
}
