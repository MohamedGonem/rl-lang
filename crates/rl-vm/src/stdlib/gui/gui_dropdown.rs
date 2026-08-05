use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{
            GuiHandle, SelectState,
            common::{attach_child, extract_string_array, insert_handle, require_window},
        },
        macros::{verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(
    eval: &mut Vm,
    window: VmValue,
    options: VmValue,
    x: i64,
    y: i64,
    width: i64,
) -> VmValue {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_dropdown") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_dropdown") {
        return verr!(vs!(e));
    }
    let options = match extract_string_array(options, "gui_dropdown") {
        Ok(o) => o,
        Err(e) => return verr!(vs!(e)),
    };
    if options.is_empty() {
        return verr!(vs!(
            "gui_dropdown: options array must not be empty".to_string()
        ));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Dropdown(SelectState {
            window: window_id,
            options,
            selected: 0,
            x: x as f32,
            y: y as f32,
            width: width.max(1) as f32,
            visible: true,
            on_change: None,
            z: 0,
        }),
    );
    let VmValue::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
