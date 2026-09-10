use crate::entry::FnEntry;

pub static GUI_IS_CHECKED: FnEntry = FnEntry {
    signature: "gui_is_checked(handle)",
    description: "returns whether the checkbox `handle` is currently checked",
    example: r#"get std::gui::gui_window
get std::gui::gui_checkbox
get std::gui::gui_is_checked

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle checkbox = result_unwrap(gui_checkbox(window, "Agree", 20, 20))
dec bool checked = result_unwrap(gui_is_checked(checkbox))"#,
    expected_output: None,
    returns: "result[bool]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a checkbox"),
    see_also: &["gui_set_checked", "gui_on_change", "gui_checkbox"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
