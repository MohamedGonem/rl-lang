use crate::entry::FnEntry;

pub static GUI_WINDOW_SET_BACKGROUND: FnEntry = FnEntry {
    signature: "gui_window_set_background(window, r, g, b)",
    description: "changes the background fill color of `window`'s central panel to the given RGB color (each channel 0-255). Takes effect on the next `gui_run` frame if the window is already open",
    example: r#"get std::gui::gui_window
get std::gui::gui_window_set_background

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_window_set_background(window, 30, 60, 90)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) for an unknown handle, a handle that isn't a window, or a channel outside 0-255",
    ),
    see_also: &["gui_window", "gui_window_set_title", "gui_run"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
