use crate::entry::FnEntry;

pub static GUI_SLIDER: FnEntry = FnEntry {
    signature: "gui_slider(window, min, max, x, y, width)",
    description: "adds a draggable slider to `window` ranging from `min` to `max` (both `float`), positioned at `(x, y)` and `width` pixels wide (`width` clamped to a minimum of 1), and returns a handle to it. Starts at `min`. Read the current position with `gui_get_value`, set it programmatically with `gui_set_value` (which clamps to `[min, max]`), or react to drags with `gui_on_change` (the callback receives the new value as a `float`). For a compact draggable/typeable number field instead of a slider bar, see `gui_number_input`",
    example: r#"get std::gui::gui_window
get std::gui::gui_slider

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle slider = result_unwrap(gui_slider(window, 0.0, 100.0, 20, 20, 200))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, is a handle that isn't a window, or if `min` is not less than `max`",
    ),
    see_also: &[
        "gui_number_input",
        "gui_get_value",
        "gui_set_value",
        "gui_on_change",
    ],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
