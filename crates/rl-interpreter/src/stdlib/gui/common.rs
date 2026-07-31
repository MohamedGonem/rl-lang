use crate::{evaluator::Evaluator, stdlib::gui::GuiHandle, values::Value};
use rl_ast::statements::HandleKind;

pub fn insert_handle(eval: &mut Evaluator, handle: GuiHandle) -> Value {
    let id = eval.gui_next_handle;
    eval.gui_next_handle += 1;
    eval.gui_handles.insert(id, handle);
    Value::Handle {
        kind: HandleKind::Gui,
        id,
    }
}

pub fn require_window(eval: &Evaluator, window_id: u64, fn_name: &str) -> Result<(), String> {
    match eval.gui_handles.get(&window_id) {
        Some(GuiHandle::Window(_)) => Ok(()),
        Some(_) => Err(format!("{}: handle {} is not a window", fn_name, window_id)),
        None => Err(format!("{}: unknown handle {}", fn_name, window_id)),
    }
}

pub fn attach_child(eval: &mut Evaluator, window_id: u64, child_id: u64) {
    if let Some(GuiHandle::Window(w)) = eval.gui_handles.get_mut(&window_id) {
        w.children.push(child_id);
    }
}

/// Extracts `array[string]` into a `Vec<String>`, for `gui_dropdown`/`gui_radio_group`.
pub fn extract_string_array(value: Value, name: &str) -> Result<Vec<String>, String> {
    match value {
        Value::Values { items, .. } => items
            .into_iter()
            .map(|v| crate::stdlib::common::extract_string(v, name))
            .collect(),
        other => Err(format!(
            "{}: expected array[string], got {}",
            name,
            other.type_name()
        )),
    }
}

/// Extracts `array[int]` into a `Vec<u8>`, validating every element is a byte
/// (0-255). Used by `gui_window_set_icon` for raw RGBA pixel data.
pub fn extract_byte_array(value: Value, name: &str) -> Result<Vec<u8>, String> {
    match value {
        Value::Values { items, .. } => items
            .into_iter()
            .map(|v| {
                let n = crate::stdlib::common::extract_int(v, name)?;
                if !(0..=255).contains(&n) {
                    return Err(format!(
                        "{}: byte value {} is out of range (must be 0-255)",
                        name, n
                    ));
                }
                Ok(n as u8)
            })
            .collect(),
        other => Err(format!(
            "{}: expected array[int], got {}",
            name,
            other.type_name()
        )),
    }
}
