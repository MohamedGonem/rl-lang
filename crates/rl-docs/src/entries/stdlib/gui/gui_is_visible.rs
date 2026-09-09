use crate::entry::FnEntry;

pub static GUI_IS_VISIBLE: FnEntry = FnEntry {
    signature: "gui_is_visible(handle)",
    description: "returns whether `handle` is currently visible. Works on any handle kind - a window or any widget",
    example: r#"get std::gui::gui_window
get std::gui::gui_is_visible

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec bool visible = result_unwrap(gui_is_visible(window))"#,
    expected_output: None,
    returns: "result[bool]",
    errors: Some("err(string) for an unknown handle"),
    see_also: &["gui_set_visible"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
