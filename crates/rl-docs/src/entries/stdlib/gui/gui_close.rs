use crate::entry::FnEntry;

pub static GUI_CLOSE: FnEntry = FnEntry {
    signature: "gui_close(window)",
    description: "tears down `window` and every widget attached to it, removing them all from the handle registry immediately - all of those handles become invalid right away, not just on the next frame. If `window` has an `on_close` callback registered, it's called (with no arguments) before removal. Called from inside a callback while `gui_run` is pumping its event loop: for the window passed to `gui_run` itself, this is what makes the native OS window close on the following frame, since that check happens once per frame; for any other open window, it closes as soon as `gui_run`'s per-frame loop stops finding its handle - both take effect on the next frame, not immediately. Called before `gui_run` has been invoked, it just deletes the window outright, so a later `gui_run(window)` call on that same handle would fail with an unknown-handle error",
    example: r#"get std::gui::gui_window
get std::gui::gui_close

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_close(window)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a window"),
    see_also: &["gui_run", "gui_on_close", "gui_quit", "gui_remove"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
