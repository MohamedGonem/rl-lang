use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            CheckboxState, GuiHandle,
            common::{attach_child, insert_handle, require_window},
        },
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, label: String, x: i64, y: i64) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_checkbox") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if let Err(e) = require_window(eval, window_id, "gui_checkbox") {
        return verr!(vs!(e));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Checkbox(CheckboxState {
            window: window_id,
            label,
            x: x as f32,
            y: y as f32,
            visible: true,
            checked: false,
            on_change: None,
        }),
    );

    let Value::Handle {
        id: checkbox_id, ..
    } = handle
    else {
        unreachable!()
    };
    attach_child(eval, window_id, checkbox_id);

    vok!(handle)
}
