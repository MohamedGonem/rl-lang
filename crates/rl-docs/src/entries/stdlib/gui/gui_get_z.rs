use crate::entry::FnEntry;

pub static GUI_GET_Z: FnEntry = FnEntry {
    signature: "gui_get_z(handle)",
    description: "returns `handle`'s current draw order (z-level) among its window's widgets, as an int. Every widget starts at 0. `handle` must be a widget, not a window itself",
    example: r#"get std::gui::gui_window
get std::gui::gui_button
get std::gui::gui_get_z

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle button = result_unwrap(gui_button(window, "Click me", 20, 20))
dec int z = result_unwrap(gui_get_z(button))"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("err(string) for an unknown handle, or a handle that's a window"),
    see_also: &["gui_set_z", "gui_get_pos"],
    since: Some("v0.4.0"),
};
