use crate::entry::FnEntry;

pub static GUI_TEXTBOX: FnEntry = FnEntry {
    signature: "gui_textbox(window, text, x, y, width)",
    description: "adds a single-line editable text field to `window`, pre-filled with `text`, positioned at `(x, y)` and `width` pixels wide (`width` clamped to a minimum of 1), and returns a handle to it. Edits the user types are written straight back into the widget's own state on the next `gui_run` frame - read them with `gui_get_text`, or react to every keystroke with `gui_on_change`, or react only when the user presses Enter with `gui_on_submit`. For a multiline text area instead, use `gui_textarea`",
    example: r#"get std::gui::gui_window
get std::gui::gui_textbox

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle textbox = result_unwrap(gui_textbox(window, "", 20, 20, 200))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &[
        "gui_textarea",
        "gui_get_text",
        "gui_set_text",
        "gui_on_change",
        "gui_on_submit",
        "gui_set_pos",
    ],
    since: Some("v0.4.0"),
};
