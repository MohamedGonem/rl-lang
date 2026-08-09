use crate::entry::FnEntry;

pub static GUI_WINDOW_SET_DECORATED: FnEntry = FnEntry {
    signature: "gui_window_set_decorated(window, decorated)",
    description: "shows or hides the native title bar and window borders of `window`, depending on `decorated`. Takes effect on the next `gui_run` frame if the window is already open",
    example: r#"get std::gui::gui_window
get std::gui::gui_window_set_decorated

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_window_set_decorated(window, false)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a window"),
    see_also: &["gui_window", "gui_window_set_icon", "gui_run"],
    since: Some("v0.4.0"),
};
