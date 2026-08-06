use crate::entry::FnEntry;

pub static GUI_NUMBER_INPUT: FnEntry = FnEntry {
    signature: "gui_number_input(window, value, min, max, x, y)",
    description: "adds a compact draggable/typeable number field to `window`, starting at `value` (clamped to `[min, max]`), positioned at `(x, y)`, and returns a handle to it. Drag it to change the value continuously, or click it to type a value directly. Shares its underlying state with `gui_slider`, so every existing slider function works on it too: `gui_get_value`, `gui_set_value` (clamps to `[min, max]`), `gui_set_visible`, `gui_is_visible`, `gui_set_pos`, `gui_get_pos`, `gui_remove`, and `gui_on_change` (the callback receives the new value as a `float`). Auto-sized to its content rather than taking a `width` - for a full-width draggable bar instead, use `gui_slider`",
    example: r#"get std::gui::gui_window
get std::gui::gui_number_input

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle age = result_unwrap(gui_number_input(window, 18.0, 0.0, 120.0, 20, 20))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, is a handle that isn't a window, or if `min` is not less than `max`",
    ),
    see_also: &[
        "gui_slider",
        "gui_get_value",
        "gui_set_value",
        "gui_on_change",
    ],
    since: Some("v0.4.0"),
};
