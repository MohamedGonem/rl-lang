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

pub fn func(
    eval: &mut Vm,
    window: VmValue,
    min: f64,
    max: f64,
    x: i64,
    y: i64,
    width: i64,
) -> VmValue {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_slider") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_slider") {
        return verr!(vs!(e));
    }

    if !(min < max) {
        return verr!(vs!(format!(
            "gui_slider: min ({}) must be less than max ({})",
            min, max
        )));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Slider(SliderState {
            window: window_id,
            value: min,
            min,
            max,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
            drag_only: false,
            z: 0,
        }),
    );
    let VmValue::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
