use crate::entry::FnEntry;

pub static GUI_QUIT: FnEntry = FnEntry {
    signature: "gui_quit()",
    description: "requests that the currently-running `gui_run` event loop stop, meant to be called from inside a `gui_on_click`/`gui_on_change` callback. Unlike `gui_close`, it takes no window handle and doesn't remove anything from the registry immediately - it just sets a flag that `gui_run`'s frame loop checks (and clears) right after that frame's callbacks finish, so the native window closes cleanly on the next frame instead of mid-callback. Calling it outside of `gui_run` has no visible effect, since nothing is polling the flag",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_on_click
get std::gui::gui_quit
get std::gui::gui_run

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle button = result_unwrap(gui_button(window, "Quit", 20, 20))

gui_on_click(button, fn() {
    gui_quit()
})?

gui_run(window)?"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["gui_close", "gui_run"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
