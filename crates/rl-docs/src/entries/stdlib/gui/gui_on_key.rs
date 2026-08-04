use crate::entry::FnEntry;

pub static GUI_ON_KEY: FnEntry = FnEntry {
    signature: "gui_on_key(window, function)",
    description: "registers `function` as the key-press callback for `window`, replacing any callback set earlier. `function` is called once with the pressed key's name (string, e.g. \"Enter\", \"Escape\", \"A\", \"ArrowUp\") for every key press while `window` has focus - held keys only fire once, not repeatedly. This is independent of individual widgets' own key handling (e.g. typing into a focused textbox still updates its text as normal) - it's a way to react to keys like Escape that no widget otherwise handles",
    example: r#"get std::gui::gui_window
get std::gui::gui_on_key
get std::gui::gui_close

dec handle window = result_unwrap(gui_window("My App", 400, 300))

gui_on_key(window, fn(string key) {
    if (key == "Escape") {
        gui_close(window)?
    }
})?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) if `function` isn't a function/lambda, if `window` is an unknown handle, or if `window` isn't a window",
    ),
    see_also: &["gui_window", "gui_on_submit", "gui_on_close"],
    since: Some("v0.4.0"),
};
