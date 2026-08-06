use crate::entry::FnEntry;

pub static GUI_CHECKBOX: FnEntry = FnEntry {
    signature: "gui_checkbox(window, label, x, y)",
    description: "adds a checkbox labeled `label` to `window`, positioned at absolute pixel coordinates `(x, y)`, and returns a handle to it. Starts unchecked; use `gui_set_checked` to set it programmatically or `gui_on_change` to react to clicks (the callback receives the new `bool` state)",
    example: r#"get std::gui::gui_window
get std::gui::gui_checkbox

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle checkbox = result_unwrap(gui_checkbox(window, "Enable feature", 20, 20))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &["gui_is_checked", "gui_set_checked", "gui_on_change"],
    since: Some("v0.4.0"),
};
