use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{
            GuiHandle, SliderState,
            common::{attach_child, insert_handle, require_window},
        },
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

/// A compact draggable/typeable number field. Shares `SliderState`/
/// `GuiHandle::Slider` with `gui_slider` (distinguished only by
/// `drag_only: true`), so it works automatically with every existing
/// slider function - `gui_get_value`, `gui_set_value`, `gui_set_visible`,
/// `gui_set_pos`, `gui_remove`, and `gui_on_change`.
pub fn func(
    eval: &mut Vm,
    window: VmValue,
    value: f64,
    min: f64,
    max: f64,
    x: i64,
    y: i64,
) -> VmValue {
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
            z: 0,
        }),
    );
    let VmValue::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
