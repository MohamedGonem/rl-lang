use crate::entry::FnEntry;

pub static GUI_ON_CLICK: FnEntry = FnEntry {
    signature: "gui_on_click(handle, function)",
    description: "registers `function` as the click callback for the button `handle`, replacing any callback set earlier. `function` is called with no arguments each time the user clicks the button during `gui_run`, after that frame's render pass has fully finished - it's free to call any other `std::gui` function, including mutating other widgets, calling `gui_close`, or calling `gui_quit`",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_label
get std::gui::gui_on_click
get std::gui::gui_set_text

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle label = result_unwrap(gui_label(window, "0 clicks", 20, 20))
dec handle button = result_unwrap(gui_button(window, "Click me", 20, 50))
dec int count = 0

gui_on_click(button, fn() {
    count = count + 1
    gui_set_text(label, "{count} clicks")?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, or if `handle` is an unknown handle or isn't a button",
    ),
    see_also: &["gui_button", "gui_on_change", "gui_run"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
