use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{
            GuiHandle, ProgressState,
            common::{attach_child, insert_handle, require_window},
        },
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, window: VmValue, x: i64, y: i64, width: i64) -> VmValue {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_progress_bar") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_progress_bar") {
        return verr!(vs!(e));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::ProgressBar(ProgressState {
            window: window_id,
            value: 0.0,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            z: 0,
        }),
    );
    let VmValue::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
