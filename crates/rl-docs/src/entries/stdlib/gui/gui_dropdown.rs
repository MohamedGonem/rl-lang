use crate::entry::FnEntry;

pub static GUI_DROPDOWN: FnEntry = FnEntry {
    signature: "gui_dropdown(window, options, x, y, width)",
    description: "adds a dropdown (combo box) to `window` offering `options` (an `array[string]`, which must not be empty), positioned at `(x, y)` and `width` pixels wide (`width` clamped to a minimum of 1), and returns a handle to it. Starts with index `0` selected. Use `gui_get_selected_index`/`gui_get_selected` to read the current choice and `gui_on_change` to react to it (the callback receives the new selected index as an `int`)",
    example: r#"get std::gui::gui_window
get std::gui::gui_dropdown

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle dropdown = result_unwrap(
    gui_dropdown(window, ["Small", "Medium", "Large"], 20, 20, 150)
)"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle or isn't a window, if `options` isn't an `array[string]`, or if `options` is empty",
    ),
    see_also: &[
        "gui_get_selected",
        "gui_get_selected_index",
        "gui_set_selected_index",
        "gui_on_change",
    ],
    since: Some("v0.4.1"),
    deprecated: None,
    updated: Some("v0.4.1"),
};
