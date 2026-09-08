use crate::entry::FnEntry;

pub static GUI_GET_SELECTED: FnEntry = FnEntry {
    signature: "gui_get_selected(handle)",
    description: "returns the currently-selected option's text (not its index - see `gui_get_selected_index` for that) for the dropdown or radio group `handle`",
    example: r#"get std::gui::gui_window
get std::gui::gui_dropdown
get std::gui::gui_get_selected

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle dropdown = result_unwrap(gui_dropdown(window, ["A", "B", "C"], 20, 20, 150))
dec string choice = result_unwrap(gui_get_selected(dropdown))"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        "err(string) for an unknown handle, or a handle that has no selection (anything but a dropdown or radio group)",
    ),
    see_also: &["gui_get_selected_index", "gui_set_selected_index"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
