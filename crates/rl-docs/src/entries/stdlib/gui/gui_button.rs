use crate::entry::FnEntry;

pub static GUI_BUTTON: FnEntry = FnEntry {
    signature: "gui_button(window, label, x, y)",
    description: "adds a clickable button labeled `label` to `window`, positioned at absolute pixel coordinates `(x, y)` within that window, and returns a handle to it. Use `gui_on_click` to attach a callback",
    example: r#"get std::gui::gui_window
get std::gui::gui_button

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle button = result_unwrap(gui_button(window, "Click me", 20, 20))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &["gui_on_click", "gui_set_text", "gui_set_pos", "gui_remove"],
    since: Some("v0.4.0"),
};
