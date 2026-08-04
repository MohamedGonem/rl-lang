use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            GuiHandle, SliderState,
            common::{attach_child, insert_handle, require_window},
        },
    },
    values::Value,
};

pub fn func(
    eval: &mut Evaluator,
    window: Value,
    value: f64,
    min: f64,
    max: f64,
    x: i64,
    y: i64,
) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_number_input") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_number_input") {
        return verr!(vs!(e));
    }

    if !(min < max) {
        return verr!(vs!(format!(
            "gui_number_input: min ({}) must be less than max ({})",
            min, max
        )));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Slider(SliderState {
            window: window_id,
            value: value.clamp(min, max),
            min,
            max,
            x: x as f32,
            y: y as f32,
            width: 0.0, // unused for drag_only widgets - DragValue auto-sizes
            visible: true,
            on_change: None,
            drag_only: true,
        }),
    );
    let Value::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
