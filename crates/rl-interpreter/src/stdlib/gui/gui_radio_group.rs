use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            GuiHandle, SelectState,
            common::{attach_child, extract_string_array, insert_handle, require_window},
        },
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, options: Value, x: i64, y: i64) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_radio_group") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_radio_group") {
        return verr!(vs!(e));
    }
    let options = match extract_string_array(options, "gui_radio_group") {
        Ok(o) => o,
        Err(e) => return verr!(vs!(e)),
    };
    if options.is_empty() {
        return verr!(vs!(
            "gui_radio_group: options array must not be empty".to_string()
        ));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::RadioGroup(SelectState {
            window: window_id,
            options,
            selected: 0,
            x: x as f32,
            y: y as f32,
            width: 0.0,
            visible: true,
            on_change: None,
        }),
    );
    let Value::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
