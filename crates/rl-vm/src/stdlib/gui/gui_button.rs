use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{
            ButtonState, GuiHandle,
            common::{attach_child, insert_handle, require_window},
        },
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, window: VmValue, label: String, x: i64, y: i64) -> VmValue {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_button") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if let Err(e) = require_window(eval, window_id, "gui_button") {
        return verr!(vs!(e));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Button(ButtonState {
            window: window_id,
            label,
            x: x as f32,
            y: y as f32,
            visible: true,
            on_click: None,
            z: 0,
        }),
    );

    let VmValue::Handle { id: button_id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, button_id);

    vok!(handle)
}
