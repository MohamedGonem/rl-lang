use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            GuiHandle, TextboxState,
            common::{attach_child, insert_handle, require_window},
        },
    },
    values::Value,
};

pub fn func(
    eval: &mut Evaluator,
    window: Value,
    text: String,
    x: i64,
    y: i64,
    width: i64,
) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_textbox") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if let Err(e) = require_window(eval, window_id, "gui_textbox") {
        return verr!(vs!(e));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Textbox(TextboxState {
            window: window_id,
            text,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
        }),
    );

    let Value::Handle { id: textbox_id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, textbox_id);

    vok!(handle)
}
