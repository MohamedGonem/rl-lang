use crate::entry::FnEntry;

pub static GUI_TEXTBOX: FnEntry = FnEntry {
    signature: "gui_textbox(window, text, x, y, width)",
    description: "adds a single-line editable text field to `window`, pre-filled with `text`, positioned at `(x, y)` and `width` pixels wide (`width` clamped to a minimum of 1), and returns a handle to it. Edits the user types are written straight back into the widget's own state on the next `gui_run` frame - read them with `gui_get_text`. There's no `gui_on_change` support for textboxes in this version: passing a textbox handle to `gui_on_change` returns an error, so polling `gui_get_text` is currently the only way to observe edits",
    example: r#"get std::gui::gui_window
get std::gui::gui_textbox

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle textbox = result_unwrap(gui_textbox(window, "", 20, 20, 200))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &["gui_get_text", "gui_set_text", "gui_set_pos"],
    since: Some("v0.4.0"),
};
