use crate::entry::FnEntry;

pub static GUI_SET_POS: FnEntry = FnEntry {
    signature: "gui_set_pos(handle, x, y)",
    description: "moves `handle` to absolute pixel coordinates `(x, y)` within its window. `handle` must be a widget, not a window itself - windows are positioned by the OS window manager, not by this API",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_set_pos

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle button = result_unwrap(gui_button(window, "Move me", 20, 20))
gui_set_pos(button, 100, 150)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that's a window"),
    see_also: &["gui_get_pos", "gui_set_z"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
