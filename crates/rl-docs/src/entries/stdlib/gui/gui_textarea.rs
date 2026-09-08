use crate::entry::FnEntry;

pub static GUI_TEXTAREA: FnEntry = FnEntry {
    signature: "gui_textarea(window, text, x, y, width, height)",
    description: "adds a multiline editable text field to `window`, pre-filled with `text`, positioned at `(x, y)`, `width` by `height` pixels (both clamped to a minimum of 1), and returns a handle to it. Shares its underlying state with `gui_textbox`, so every existing textbox function works on it too: `gui_get_text`, `gui_set_text`, `gui_set_visible`, `gui_is_visible`, `gui_set_pos`, `gui_get_pos`, `gui_remove`, and `gui_on_change` (fires on every keystroke). The one exception is `gui_on_submit`: Enter inserts a newline in a textarea instead of submitting, so registering an `on_submit` callback on a textarea handle returns an error - use a button for an explicit submit action instead",
    example: r#"get std::gui::gui_window
get std::gui::gui_textarea

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle notes = result_unwrap(gui_textarea(window, "", 20, 20, 300, 150))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &[
        "gui_textbox",
        "gui_get_text",
        "gui_set_text",
        "gui_on_change",
    ],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
