use crate::entry::FnEntry;

pub static GUI_CLOSE: FnEntry = FnEntry {
    signature: "gui_close(window)",
    description: "tears down `window` and every widget attached to it, removing them all from the handle registry immediately - all of those handles become invalid right away, not just on the next frame. Called from inside a callback while `gui_run(window)` is pumping its event loop, this is what makes the native OS window close on the following frame, since `gui_run` checks each frame whether its window handle still exists. Called before `gui_run(window)` has been invoked, it just deletes the window outright, so the later `gui_run(window)` call would fail with an unknown-handle error",
    example: r#"get std::gui::gui_window
get std::gui::gui_close

dec handle window = result_unwrap(gui_window("My App", 400, 300))
gui_close(window)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a window"),
    see_also: &["gui_run", "gui_quit", "gui_remove"],
    since: Some("v0.4.0"),
};
