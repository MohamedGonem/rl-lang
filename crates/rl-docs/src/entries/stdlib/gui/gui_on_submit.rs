use crate::entry::FnEntry;

pub static GUI_ON_SUBMIT: FnEntry = FnEntry {
    signature: "gui_on_submit(textbox, function)",
    description: "registers `function` as the submit callback for `textbox`, replacing any callback set earlier. `function` is called with the textbox's current text (string) when Enter is pressed while the textbox has focus. Unlike `gui_on_change`, this fires once per Enter press rather than on every keystroke, making it suitable for a search box or a single-line form field",
    example: r#"get std::gui::gui_window
get std::gui::gui_textbox
get std::gui::gui_label
get std::gui::gui_on_submit
get std::gui::gui_set_text

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle search = result_unwrap(gui_textbox(window, "", 20, 20, 200))
dec handle result = result_unwrap(gui_label(window, "", 20, 50))

gui_on_submit(search, fn(string text) {
    gui_set_text(result, "searching for: {text}")?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, if `textbox` is an unknown handle, or if `textbox` isn't a textbox",
    ),
    see_also: &["gui_textbox", "gui_on_change", "gui_on_key"],
    since: Some("v0.4.0"),
};
