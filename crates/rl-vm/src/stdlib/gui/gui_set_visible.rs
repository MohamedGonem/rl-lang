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

pub fn func(eval: &mut Vm, handle: VmValue, visible: bool) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_set_visible") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.gui_handles.get_mut(&id) {
        Some(GuiHandle::Window(w)) => w.visible = visible,
        Some(GuiHandle::Button(b)) => b.visible = visible,
        Some(GuiHandle::Label(l)) => l.visible = visible,
        Some(GuiHandle::Checkbox(c)) => c.visible = visible,
        Some(GuiHandle::Textbox(t)) => t.visible = visible,
        Some(GuiHandle::Dropdown(s)) => s.visible = visible,
        Some(GuiHandle::RadioGroup(s)) => s.visible = visible,
        Some(GuiHandle::Slider(s)) => s.visible = visible,
        Some(GuiHandle::ProgressBar(p)) => p.visible = visible,
        Some(GuiHandle::Separator(s)) => s.visible = visible,
        Some(GuiHandle::Image(i)) => i.visible = visible,
        None => return verr!(vs!(format!("gui_set_visible: unknown handle {}", id))),
    }

    vok!(vnl!())
}
