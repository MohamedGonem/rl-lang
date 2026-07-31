use crate::entry::FnEntry;

pub static GUI_RUN: FnEntry = FnEntry {
    signature: "gui_run(window)",
    description: "opens `window` as a native OS window and blocks the calling thread, running its event loop until the window closes. Each frame: every visible child widget is drawn at its current position/state, then any button clicks, checkbox toggles, dropdown/radio/slider changes, and textbox edits from that frame are applied back to the widget's own state and their `gui_on_click`/`gui_on_change` callbacks are invoked (with a fully-finished render pass behind them, so callbacks can freely call other `std::gui` functions). The window closes when the user closes it via the OS chrome, or when a callback running during `gui_run` calls `gui_close(window)` or `gui_quit()`. Only one `gui_run` call can be in flight at a time - it owns the interpreter for its duration",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_on_click
get std::gui::gui_close
get std::gui::gui_run

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle button = result_unwrap(gui_button(window, "Close", 20, 20))

gui_on_click(button, fn() {
    gui_close(window)?
})?

gui_run(window)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `window` is an unknown handle or isn't a window, or if the underlying native windowing backend fails to start",
    ),
    see_also: &["gui_window", "gui_close", "gui_quit"],
    since: Some("v0.4.0"),
};
