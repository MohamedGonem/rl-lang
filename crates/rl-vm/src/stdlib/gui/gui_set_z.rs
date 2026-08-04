use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

/// Sets `handle`'s draw order among its window's widgets. Higher `z` draws
/// on top of lower `z` when positions overlap; widgets with equal `z` draw
/// in creation order (later created = on top).
pub fn func(eval: &mut Vm, handle: VmValue, z: i64) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_z") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };
    let z = z as i32;

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Button(w)) => w.z = z,
        Some(GuiHandle::Label(w)) => w.z = z,
        Some(GuiHandle::Checkbox(w)) => w.z = z,
        Some(GuiHandle::Textbox(w)) => w.z = z,
        Some(GuiHandle::Dropdown(w)) => w.z = z,
        Some(GuiHandle::RadioGroup(w)) => w.z = z,
        Some(GuiHandle::Slider(w)) => w.z = z,
        Some(GuiHandle::ProgressBar(w)) => w.z = z,
        Some(GuiHandle::Separator(w)) => w.z = z,
        Some(GuiHandle::Image(w)) => w.z = z,
        Some(GuiHandle::Window(_)) => {
            return verr!(vs!(format!(
                "gui_set_z: handle {} is a window and has no z-level - z-level controls draw order between widgets inside a window, not between windows",
                id
            )));
        }
        None => return verr!(vs!(format!("gui_set_z: unknown handle {}", id))),
    }

    vok!(vnl!())
}
