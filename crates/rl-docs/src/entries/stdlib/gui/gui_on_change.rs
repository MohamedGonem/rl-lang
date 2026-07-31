use crate::entry::FnEntry;

pub static GUI_ON_CHANGE: FnEntry = FnEntry {
    signature: "gui_on_change(handle, function)",
    description: "registers `function` as the change callback for `handle`, replacing any callback set earlier. Supported on checkboxes, dropdowns, radio groups, and sliders - the argument type `function` is called with depends on which: `bool` for a checkbox, `int` (the new selected index) for a dropdown or radio group, `float` (the new value) for a slider. It fires only for user-driven changes during `gui_run` (dragging, clicking, selecting), never for `gui_set_checked`/`gui_set_selected_index`/`gui_set_value` calls. Textboxes and progress bars don't support `gui_on_change` at all - a textbox's edits are only observable by polling `gui_get_text`",
    example: r#"get std::gui::gui_window
get std::gui::gui_slider
get std::gui::gui_label
get std::gui::gui_on_change
get std::gui::gui_set_text

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle label = result_unwrap(gui_label(window, "0", 20, 20))
dec handle slider = result_unwrap(gui_slider(window, 0.0, 100.0, 20, 50, 200))

gui_on_change(slider, fn(float value) {
    gui_set_text(label, "{value}")?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, if `handle` is an unknown handle, or if `handle`'s widget kind doesn't support change events (button, label, textbox, progress_bar, window)",
    ),
    see_also: &[
        "gui_checkbox",
        "gui_dropdown",
        "gui_radio_group",
        "gui_slider",
        "gui_on_click",
    ],
    since: Some("v0.4.0"),
};
