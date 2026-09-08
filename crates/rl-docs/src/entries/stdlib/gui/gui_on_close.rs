use crate::entry::FnEntry;

pub static GUI_ON_CLOSE: FnEntry = FnEntry {
    signature: "gui_on_close(window, function)",
    description: "registers `function` as the close callback for `window`, replacing any callback set earlier. `function` is called with no arguments right before `window`'s handle (and all its widgets' handles) are removed - whether the window was closed via `gui_close`, or by the user clicking the window's native close button. Not fired by `gui_quit`, which tears the whole application down immediately without closing windows individually",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_on_close
get std::gui::gui_set_visible

dec handle main = result_unwrap(gui_window("Main", 400, 300))
dec handle settings = result_unwrap(gui_window("Settings", 300, 200))
dec handle open_settings_btn = result_unwrap(gui_button(main, "Open Settings", 20, 20))

gui_on_close(settings, fn() {
    gui_set_visible(open_settings_btn, true)?
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, if `window` is an unknown handle, or if `window` isn't a window",
    ),
    see_also: &["gui_window", "gui_close", "gui_on_key"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
