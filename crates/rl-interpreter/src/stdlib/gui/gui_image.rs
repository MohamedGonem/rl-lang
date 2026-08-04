use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vok, vs},
        gui::{
            GuiHandle, ImageState,
            common::{attach_child, extract_byte_array, insert_handle, require_window},
        },
    },
    values::Value,
};

/// callers must supply already-decoded pixels.
pub fn func(
    eval: &mut Evaluator,
    window: Value,
    width: i64,
    height: i64,
    rgba: Value,
    x: i64,
    y: i64,
) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_image") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    if let Err(e) = require_window(eval, window_id, "gui_image") {
        return verr!(vs!(e));
    }

    if width <= 0 || height <= 0 {
        return verr!(vs!(format!(
            "gui_image: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let rgba_bytes = match extract_byte_array(rgba, "gui_image") {
        Ok(bytes) => bytes,
        Err(e) => return verr!(vs!(e)),
    };

    let expected_len = width as usize * height as usize * 4;
    if rgba_bytes.len() != expected_len {
        return verr!(vs!(format!(
            "gui_image: expected {} rgba bytes for a {}x{} image, got {}",
            expected_len,
            width,
            height,
            rgba_bytes.len()
        )));
    }

    let handle = insert_handle(
        eval,
        GuiHandle::Image(ImageState {
            window: window_id,
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
            visible: true,
            rgba: (width as u32, height as u32, std::sync::Arc::new(rgba_bytes)),
        }),
    );
    let Value::Handle { id, .. } = handle else {
        unreachable!()
    };
    attach_child(eval, window_id, id);

    vok!(handle)
}
