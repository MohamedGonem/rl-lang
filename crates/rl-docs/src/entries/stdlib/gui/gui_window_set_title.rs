use crate::entry::FnEntry;

pub static GUI_WINDOW_SET_TITLE: FnEntry = FnEntry {
    signature: "gui_window_set_title(window, title)",
    description: "changes the title bar text of `window` to `title`. Takes effect on the next `gui_run` frame if the window is already open",
    example: r#"get std::gui::gui_window
get std::gui::gui_window_set_title

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_window_set_title(window, "My App - unsaved changes")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a window"),
    see_also: &["gui_window", "gui_run"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
