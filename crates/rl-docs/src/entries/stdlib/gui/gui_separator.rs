use crate::entry::FnEntry;

pub static GUI_SEPARATOR: FnEntry = FnEntry {
    signature: "gui_separator(window, x, y, width)",
    description: "adds a horizontal divider line to `window`, positioned at `(x, y)` and `width` pixels wide (`width` clamped to a minimum of 1), and returns a handle to it. Purely visual - no interaction, no `gui_on_change`. Supports `gui_set_visible`, `gui_is_visible`, `gui_set_pos`, `gui_get_pos`, and `gui_remove` like other widgets",
    example: r#"get std::gui::gui_window
get std::gui::gui_separator

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle divider = result_unwrap(gui_separator(window, 20, 60, 200))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &["gui_set_visible", "gui_set_pos", "gui_remove"],
    since: Some("v0.4.0"),
};
