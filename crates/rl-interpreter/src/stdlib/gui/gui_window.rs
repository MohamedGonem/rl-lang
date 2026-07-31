use crate::{
    evaluator::Evaluator,
    stdlib::gui::{GuiHandle, WindowState, common::insert_handle},
    values::Value,
};

use crate::stdlib::common::vok;

pub fn func(eval: &mut Evaluator, title: String, width: i64, height: i64) -> Value {
    let handle = insert_handle(
        eval,
        GuiHandle::Window(WindowState {
            title,
            width: width.max(1) as f32,
            height: height.max(1) as f32,
            visible: true,
            children: Vec::new(),
            background: (27, 27, 27),
            pending_size: None,
            pending_position: None,
            decorated: true,
        }),
    );
    vok!(handle)
}
