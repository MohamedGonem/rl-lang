use crate::{
    evaluator::Evaluator,
    stdlib::gui::{GuiHandle, WindowState, common::insert_handle},
    values::Value,
};

use crate::stdlib::common::vok;

pub fn func(eval: &mut Evaluator, title: String, width: i64, height: i64) -> Value {
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
