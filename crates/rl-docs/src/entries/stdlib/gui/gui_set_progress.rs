use crate::entry::FnEntry;

pub static GUI_SET_PROGRESS: FnEntry = FnEntry {
    signature: "gui_set_progress(handle, value)",
    description: "sets the fill level of the progress bar `handle` to `value`, clamped to `[0.0, 1.0]`",
    example: r#"get std::gui::gui_window
get std::gui::gui_progress_bar
get std::gui::gui_set_progress

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle bar = result_unwrap(gui_progress_bar(window, 20, 20, 200))
gui_set_progress(bar, 0.75)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a progress bar"),
    see_also: &["gui_get_progress", "gui_progress_bar"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
