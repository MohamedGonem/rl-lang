use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        gui::GuiHandle,
        macros::{vb, verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(eval: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_is_visible") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get(&id) {
        Some(GuiHandle::Window(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Button(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Label(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Checkbox(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Textbox(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Dropdown(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::RadioGroup(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Slider(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::ProgressBar(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Separator(w)) => vok!(vb!(w.visible)),
        Some(GuiHandle::Image(w)) => vok!(vb!(w.visible)),
        None => verr!(vs!(format!("gui_is_visible: unknown handle {}", id))),
    }
}
