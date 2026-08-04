use crate::{
    Vm,
    stdlib::gui::{GuiHandle, WindowState, common::insert_handle},
    values::VmValue,
};

use crate::stdlib::macros::vok;

pub fn func(eval: &mut Vm, title: String, width: i64, height: i64) -> VmValue {
    let width = width.max(1) as f32;
    let height = height.max(1) as f32;
    let handle = insert_handle(
        eval,
        GuiHandle::Window(WindowState {
            title,
            width,
            height,
            visible: true,
            children: Vec::new(),
            background: (27, 27, 27),
            pending_size: Some((width, height)),
            pending_position: None,
            decorated: true,
            icon: None,
            on_close: None,
            on_key: None,
        }),
    );
    vok!(handle)
}
