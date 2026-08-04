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

/// A multiline textarea. Shares `TextboxState`/`GuiHandle::Textbox` with
/// `gui_textbox` (distinguished only by `multiline: true`), so it works
/// automatically with every existing textbox function - `gui_get_text`,
/// `gui_set_text`, `gui_set_visible`, `gui_set_pos`, `gui_remove`, and
/// `gui_on_change`. The one exception is `gui_on_submit`: Enter inserts a
/// newline here instead of submitting, so it never fires for a textarea.
pub fn func(
    eval: &mut Evaluator,
    window: Value,
    text: String,
    x: i64,
    y: i64,
    width: i64,
    height: i64,
) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_textarea") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if let Err(e) = require_window(eval, window_id, "gui_textarea") {
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
            on_submit: None,
            multiline: true,
            height: height.max(1) as f32,
            z: 0,
        }),
    );

    let Value::Handle {
        id: textarea_id, ..
    } = handle
    else {
        unreachable!()
    };
    attach_child(eval, window_id, textarea_id);

    vok!(handle)
}
