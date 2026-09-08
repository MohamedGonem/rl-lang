use crate::entry::FnEntry;

pub static GUI_SET_Z: FnEntry = FnEntry {
    signature: "gui_set_z(handle, z)",
    description: "sets `handle`'s draw order among its window's widgets. When two widgets' positions overlap, the one with the higher `z` draws on top. Widgets with equal `z` (the default is 0 for every widget) draw in creation order, later created on top - `gui_set_z` only matters once you need to override that default ordering. `handle` must be a widget, not a window itself - z-level controls draw order between widgets inside one window, not stacking between separate windows",
    example: r#"get std::gui::gui_window
get std::gui::gui_label
get std::gui::gui_button
get std::gui::gui_set_z

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle background_label = result_unwrap(gui_label(window, "behind", 20, 20))
dec handle button = result_unwrap(gui_button(window, "in front", 20, 20))

// button was created after the label, so it already draws on top by
// default - this forces the label above it instead
gui_set_z(background_label, 1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that's a window"),
    see_also: &["gui_get_z", "gui_set_pos"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
