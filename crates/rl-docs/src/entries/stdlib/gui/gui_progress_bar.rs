use crate::entry::FnEntry;

pub static GUI_PROGRESS_BAR: FnEntry = FnEntry {
    signature: "gui_progress_bar(window, x, y, width)",
    description: "adds a progress bar to `window`, positioned at `(x, y)` and `width` pixels wide (`width` clamped to a minimum of 1), and returns a handle to it. Starts at `0.0`. It's display-only - there's no drag/click interaction, so there's no `gui_on_change` support for it. Drive it with `gui_set_progress`, which takes a `float` and clamps it to `[0.0, 1.0]`",
    example: r#"get std::gui::gui_window
get std::gui::gui_progress_bar

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle bar = result_unwrap(gui_progress_bar(window, 20, 20, 200))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, or is a handle that isn't a window",
    ),
    see_also: &["gui_set_progress", "gui_get_progress"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
