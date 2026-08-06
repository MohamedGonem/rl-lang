use crate::entry::FnEntry;

pub static GUI_GET_TEXT: FnEntry = FnEntry {
    signature: "gui_get_text(handle)",
    description: "reads the current text of `handle`, which must be a button (its label), a label, or a textbox - for a textbox this reflects any edits the user has typed as of the most recent `gui_run` frame",
    example: r#"get std::gui::gui_window
get std::gui::gui_textbox
get std::gui::gui_get_text

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle textbox = result_unwrap(gui_textbox(window, "hello", 20, 20, 200))
dec string text = result_unwrap(gui_get_text(textbox))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        "err(string) for an unknown handle, or a handle whose widget kind has no text (checkbox, dropdown, radio_group, slider, progress_bar, window)",
    ),
    see_also: &["gui_set_text", "gui_button", "gui_label", "gui_textbox"],
    since: Some("v0.4.0"),
};
