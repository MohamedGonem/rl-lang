use crate::entry::FnEntry;

pub static GUI_GET_POS: FnEntry = FnEntry {
    signature: "gui_get_pos(handle)",
    description: "returns the absolute pixel `(x, y)` position of `handle` within its window, as a tuple. `handle` must be a widget, not a window itself",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_get_pos

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle button = result_unwrap(gui_button(window, "Click me", 20, 20))
dec (int, int) pos = result_unwrap(gui_get_pos(button))"#,
    expected_output: None,
    returns: "result[(int, int)]",
    errors: Some("err(string) for an unknown handle, or a handle that's a window"),
    see_also: &["gui_set_pos", "gui_get_z"],
    since: Some("v0.4.0"),
};
