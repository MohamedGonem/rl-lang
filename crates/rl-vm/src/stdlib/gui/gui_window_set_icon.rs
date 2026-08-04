use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::{GuiHandle, common::extract_byte_array},
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, window: VmValue, width: i64, height: i64, rgba: VmValue) -> VmValue {
    let id = match extract_handle(window, HandleKind::Gui, "gui_window_set_icon") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    if width <= 0 || height <= 0 {
        return verr!(vs!(format!(
            "gui_window_set_icon: width ({}) and height ({}) must be positive",
            width, height
        )));
    }

    let rgba = match extract_byte_array(rgba, "gui_window_set_icon") {
        Ok(bytes) => bytes,
        Err(e) => return verr!(vs!(e)),
    };

    let expected_len = width as usize * height as usize * 4;
    if rgba.len() != expected_len {
        return verr!(vs!(format!(
            "gui_window_set_icon: expected {} rgba bytes for a {}x{} icon, got {}",
            expected_len,
            width,
            height,
            rgba.len()
        )));
    }

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => {
            w.icon = Some((width as u32, height as u32, rgba));
            vok!(vnl!())
        }
        Some(_) => verr!(vs!(format!(
            "gui_window_set_icon: handle {} is not a window",
            id
        ))),
        None => verr!(vs!(format!("gui_window_set_icon: unknown handle {}", id))),
    }
}
