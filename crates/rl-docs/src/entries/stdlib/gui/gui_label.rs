use crate::entry::FnEntry;

pub static GUI_LABEL: FnEntry = FnEntry {
    signature: "gui_label(window, text, x, y)",
    description: "adds a static text label showing `text` to `window`, positioned at absolute pixel coordinates `(x, y)`, and returns a handle to it. Labels have no interaction of their own - use `gui_set_text` to update the displayed text later",
    example: r#"get std::gui::gui_window
get std::gui::gui_label

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle label = result_unwrap(gui_label(window, "Hello!", 20, 20))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &["gui_set_text", "gui_get_text", "gui_set_pos"],
    since: Some("v0.4.0"),
};
