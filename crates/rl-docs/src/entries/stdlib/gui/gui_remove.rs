use crate::entry::FnEntry;

pub static GUI_REMOVE: FnEntry = FnEntry {
    signature: "gui_remove(handle)",
    description: "deletes the widget `handle`, unlinking it from its window's children and dropping it from the handle registry entirely - it disappears on the next `gui_run` frame and the handle becomes invalid. `handle` must be a widget, not a window itself - use `gui_close` to tear down a whole window (and everything in it) instead",
    example: r#"get std::gui::gui_window
get std::gui::gui_label
get std::gui::gui_remove

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle label = result_unwrap(gui_label(window, "Temporary", 20, 20))
gui_remove(label)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) for an unknown handle, or a handle that's a window (use `gui_close` instead)",
    ),
    see_also: &["gui_close", "gui_set_visible"],
    since: Some("v0.4.0"),
};
