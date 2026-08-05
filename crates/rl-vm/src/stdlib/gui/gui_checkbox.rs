use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{
            CheckboxState, GuiHandle,
            common::{attach_child, insert_handle, require_window},
        },
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, window: VmValue, label: String, x: i64, y: i64) -> VmValue {
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
            z: 0,
        }),
    );

    let VmValue::Handle {
        id: checkbox_id, ..
    } = handle
    else {
        unreachable!()
    };
    attach_child(eval, window_id, checkbox_id);

    vok!(handle)
}
