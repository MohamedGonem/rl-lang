use crate::entry::FnEntry;

pub static GUI_RUN: FnEntry = FnEntry {
    signature: "gui_run(window)",
    description: "opens `window` as a native OS window and blocks the calling thread, running the event loop until `window` closes. Every other `gui_window` handle that exists (created before `gui_run` or from within a callback) is also opened as its own native window for as long as it exists - `window` is just the one whose lifetime drives the whole event loop, not the only one shown. Each frame: every visible child widget in every open window is drawn at its current position/state, then any button clicks, checkbox toggles, dropdown/radio/slider changes, textbox edits, Enter-to-submit, and key presses from that frame are applied back to the relevant widget's state and their `gui_on_click`/`gui_on_change`/`gui_on_submit`/`gui_on_key`/`gui_on_close` callbacks are invoked (with a fully-finished render pass behind them, so callbacks can freely call other `std::gui` functions, including creating or closing other windows). `window` itself closes when the user closes it via the OS chrome, or when a callback calls `gui_close(window)`; the whole call returns once `window` closes or `gui_quit()` is called from any callback - other windows still open at that point are torn down without their `on_close` firing. Only one `gui_run` call can be in flight at a time - it owns the interpreter for its duration",
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
    see_also: &["gui_window", "gui_close", "gui_on_close", "gui_quit"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
