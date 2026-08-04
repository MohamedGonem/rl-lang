use crate::entry::FnEntry;

pub static GUI_WINDOW_SET_POS: FnEntry = FnEntry {
    signature: "gui_window_set_pos(window, x, y)",
    description: "moves `window` to screen position `(x, y)`. If the window is already open, this is applied once on the next `gui_run` frame and then does not repeat, so it won't fight the user manually dragging the window afterward",
    example: r#"get std::gui::gui_window
get std::gui::gui_window_set_pos

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_window_set_pos(window, 100, 100)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a window"),
    see_also: &["gui_window", "gui_window_set_size", "gui_run"],
    since: Some("v0.4.0"),
};
