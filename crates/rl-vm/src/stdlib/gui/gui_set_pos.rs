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

pub fn func(eval: &mut Vm, handle: VmValue, x: i64, y: i64) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_pos") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Button(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Label(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Checkbox(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Textbox(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Dropdown(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::RadioGroup(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Slider(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::ProgressBar(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Separator(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Image(w)) => {
            w.x = x as f32;
            w.y = y as f32;
        }
        Some(GuiHandle::Window(_)) => {
            return verr!(vs!(format!(
                "gui_set_pos: handle {} is a window and cannot be repositioned",
                id
            )));
        }
        None => return verr!(vs!(format!("gui_set_pos: unknown handle {}", id))),
    }

    vok!(vnl!())
}
