use crate::entry::FnEntry;

pub static GUI_WINDOW_SET_SIZE: FnEntry = FnEntry {
    signature: "gui_window_set_size(window, width, height)",
    description: "resizes `window` to `width` x `height` (logical points). If the window is already open, this is applied once on the next `gui_run` frame and then does not repeat, so it won't fight the user manually resizing the window afterward",
    example: r#"get std::gui::gui_window
get std::gui::gui_window_set_size

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_window_set_size(window, 800, 600)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a window"),
    see_also: &["gui_window", "gui_window_set_pos", "gui_run"],
    since: Some("v0.4.0"),
};
