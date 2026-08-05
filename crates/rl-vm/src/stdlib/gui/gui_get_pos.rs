use std::rc::Rc;

use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{verr, vi, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_get_pos") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let (x, y) = match eval.gui_handles.get(&id) {
        Some(GuiHandle::Button(w)) => (w.x, w.y),
        Some(GuiHandle::Label(w)) => (w.x, w.y),
        Some(GuiHandle::Checkbox(w)) => (w.x, w.y),
        Some(GuiHandle::Textbox(w)) => (w.x, w.y),
        Some(GuiHandle::Dropdown(w)) => (w.x, w.y),
        Some(GuiHandle::RadioGroup(w)) => (w.x, w.y),
        Some(GuiHandle::Slider(w)) => (w.x, w.y),
        Some(GuiHandle::ProgressBar(w)) => (w.x, w.y),
        Some(GuiHandle::Separator(w)) => (w.x, w.y),
        Some(GuiHandle::Image(w)) => (w.x, w.y),
        Some(GuiHandle::Window(_)) => {
            return verr!(vs!(format!("gui_get_pos: handle {} is a window", id)));
        }
        None => return verr!(vs!(format!("gui_get_pos: unknown handle {}", id))),
    };

    vok!(VmValue::Tuple(Rc::new(vec![vi!(x as i64), vi!(y as i64)])))
}
