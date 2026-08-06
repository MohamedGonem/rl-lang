use crate::entry::FnEntry;

pub static GUI_SET_SELECTED_INDEX: FnEntry = FnEntry {
    signature: "gui_set_selected_index(handle, index)",
    description: "sets the selected option of the dropdown or radio group `handle` to `index` programmatically. Doesn't fire the `gui_on_change` callback - that only runs in response to the user changing the selection during `gui_run`",
    example: r#"get std::gui::gui_window
get std::gui::gui_dropdown
get std::gui::gui_set_selected_index

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle dropdown = result_unwrap(gui_dropdown(window, ["A", "B", "C"], 20, 20, 150))
gui_set_selected_index(dropdown, 2)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) for an unknown handle, a handle that has no selection (anything but a dropdown or radio group), or an `index` outside `0..options.len()`",
    ),
    see_also: &["gui_get_selected_index", "gui_get_selected"],
    since: Some("v0.4.0"),
};
