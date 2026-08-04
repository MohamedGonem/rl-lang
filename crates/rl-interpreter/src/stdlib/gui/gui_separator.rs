use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            GuiHandle, SeparatorState,
            common::{attach_child, insert_handle, require_window},
        },
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, window: Value, x: i64, y: i64, width: i64) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_separator") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_separator") {
        return verr!(vs!(e));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Separator(SeparatorState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
        }),
    );
    let Value::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
