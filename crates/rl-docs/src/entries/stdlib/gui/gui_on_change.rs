use crate::entry::FnEntry;

pub static GUI_ON_CHANGE: FnEntry = FnEntry {
    signature: "gui_on_change(handle, function)",
    description: "registers `function` as the change callback for `handle`, replacing any callback set earlier. Supported on checkboxes, dropdowns, radio groups, sliders, and text fields (both `gui_textbox` and `gui_textarea`, which share the same behavior here) - the argument type `function` is called with depends on which: `bool` for a checkbox, `int` (the new selected index) for a dropdown or radio group, `float` (the new value) for a slider, `string` (the new text, fired on every edit) for a text field. It fires only for user-driven changes during `gui_run` (dragging, clicking, selecting, typing), never for `gui_set_checked`/`gui_set_selected_index`/`gui_set_value`/`gui_set_text` calls. Progress bars don't support `gui_on_change` at all. For a single-line `gui_textbox` specifically, see also `gui_on_submit` for an Enter-to-submit callback instead of one that fires on every keystroke",
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
        "err(string) if `function` isn't a function/lambda, if `handle` is an unknown handle, or if `handle`'s widget kind doesn't support change events (button, label, progress_bar, window)",
    ),
    see_also: &[
        "gui_checkbox",
        "gui_dropdown",
        "gui_radio_group",
        "gui_slider",
        "gui_textbox",
        "gui_on_submit",
        "gui_on_click",
    ],
    since: Some("v0.4.0"),
};
