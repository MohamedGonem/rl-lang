use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Gui, "gui_remove") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let window_id = match eval.gui_handles.get(&id) {
        Some(GuiHandle::Button(w)) => w.window,
        Some(GuiHandle::Label(w)) => w.window,
        Some(GuiHandle::Checkbox(w)) => w.window,
        Some(GuiHandle::Textbox(w)) => w.window,
        Some(GuiHandle::Dropdown(w)) => w.window,
        Some(GuiHandle::RadioGroup(w)) => w.window,
        Some(GuiHandle::Slider(w)) => w.window,
        Some(GuiHandle::ProgressBar(w)) => w.window,
        Some(GuiHandle::Window(_)) => {
            return verr!(vs!(format!(
                "gui_remove: handle {} is a window - use gui_close instead",
                id
            )));
        }
        None => return verr!(vs!(format!("gui_remove: unknown handle {}", id))),
    };

    if let Some(GuiHandle::Window(w)) = eval.gui_handles.get_mut(&window_id) {
        w.children.retain(|c| *c != id);
    }
    eval.gui_handles.remove(&id);

    vok!(vnl!())
}
