use crate::entry::FnEntry;

pub static GUI_GET_PROGRESS: FnEntry = FnEntry {
    signature: "gui_get_progress(handle)",
    description: "returns the current fill level of the progress bar `handle`, somewhere in `[0.0, 1.0]`",
    example: r#"get std::gui::gui_window
get std::gui::gui_progress_bar
get std::gui::gui_get_progress

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle bar = result_unwrap(gui_progress_bar(window, 20, 20, 200))
dec float level = result_unwrap(gui_get_progress(bar))"#,
    expected_output: None,
    returns: "result[float]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a progress bar"),
    see_also: &["gui_set_progress", "gui_progress_bar"],
    since: Some("v0.4.0"),
};
