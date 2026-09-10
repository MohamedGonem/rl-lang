use crate::entry::FnEntry;

pub static GUI_WINDOW: FnEntry = FnEntry {
    signature: "gui_window(title, width, height)",
    description: "creates a top-level native window `width`x`height` logical pixels with the given `title`, and returns a handle to it. The window isn't shown yet - nothing appears on screen until that handle (or one derived from it) is passed to `gui_run`. `width`/`height` are clamped to a minimum of 1. This handle is also the parent every widget function (`gui_button`, `gui_label`, etc.) attaches to; a window starts empty and `visible`",
    example: r#"get std::gui::gui_window

dec handle window = result_unwrap(gui_window("My App", 400, 300))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: None,
    see_also: &["gui_run", "gui_close", "gui_window_set_title"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
