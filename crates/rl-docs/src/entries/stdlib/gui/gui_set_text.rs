use crate::entry::FnEntry;

pub static GUI_SET_TEXT: FnEntry = FnEntry {
    signature: "gui_set_text(handle, text)",
    description: "replaces the displayed text of `handle`, which must be a button (its label), a label, or a textbox (its editable content)",
    example: r#"get std::gui::gui_window
get std::gui::gui_label
get std::gui::gui_set_text

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle label = result_unwrap(gui_label(window, "Waiting...", 20, 20))
gui_set_text(label, "Done!")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) for an unknown handle, or a handle whose widget kind has no text (checkbox, dropdown, radio_group, slider, progress_bar, window)",
    ),
    see_also: &["gui_get_text", "gui_button", "gui_label", "gui_textbox"],
    since: Some("v0.4.0"),
};
