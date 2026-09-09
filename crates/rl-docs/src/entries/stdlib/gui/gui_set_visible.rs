use crate::entry::FnEntry;

pub static GUI_SET_VISIBLE: FnEntry = FnEntry {
    signature: "gui_set_visible(handle, visible)",
    description: "shows or hides `handle`. Works on any handle kind - a window or any widget. Hiding a window doesn't close it (it's still checked by `gui_run` and still holds its children); hiding a widget just skips rendering it on the next `gui_run` frame, it stays in the registry and keeps its state",
    example: r#"get std::gui::gui_window
get std::gui::gui_label
get std::gui::gui_set_visible

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle label = result_unwrap(gui_label(window, "Hidden for now", 20, 20))
gui_set_visible(label, false)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle"),
    see_also: &["gui_is_visible", "gui_remove", "gui_close"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
