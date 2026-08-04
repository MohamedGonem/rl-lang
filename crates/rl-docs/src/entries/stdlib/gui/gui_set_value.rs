use crate::entry::FnEntry;

pub static GUI_SET_VALUE: FnEntry = FnEntry {
    signature: "gui_set_value(handle, value)",
    description: "sets the position of the slider `handle` to `value` programmatically, clamped to the slider's `[min, max]` range. Doesn't fire the `gui_on_change` callback - that only runs in response to the user dragging the slider during `gui_run`",
    example: r#"get std::gui::gui_window
get std::gui::gui_slider
get std::gui::gui_set_value

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle slider = result_unwrap(gui_slider(window, 0.0, 100.0, 20, 20, 200))
gui_set_value(slider, 50.0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a slider"),
    see_also: &["gui_get_value", "gui_on_change", "gui_slider"],
    since: Some("v0.4.0"),
};
