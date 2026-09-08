use crate::entry::FnEntry;

pub static GUI_GET_VALUE: FnEntry = FnEntry {
    signature: "gui_get_value(handle)",
    description: "returns the current position of the slider `handle`, somewhere in `[min, max]` as passed to `gui_slider`",
    example: r#"get std::gui::gui_window
get std::gui::gui_slider
get std::gui::gui_get_value

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle slider = result_unwrap(gui_slider(window, 0.0, 100.0, 20, 20, 200))
dec float value = result_unwrap(gui_get_value(slider))"#,
    expected_output: None,
    returns: "result[float]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a slider"),
    see_also: &["gui_set_value", "gui_on_change", "gui_slider"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
