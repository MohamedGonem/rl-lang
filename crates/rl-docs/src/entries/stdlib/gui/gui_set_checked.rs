use crate::entry::FnEntry;

pub static GUI_SET_CHECKED: FnEntry = FnEntry {
    signature: "gui_set_checked(handle, checked)",
    description: "sets the checked state of the checkbox `handle` programmatically. Doesn't fire the `gui_on_change` callback - that only runs in response to the user clicking the checkbox during `gui_run`",
    example: r#"get std::gui::gui_window
get std::gui::gui_checkbox
get std::gui::gui_set_checked

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle checkbox = result_unwrap(gui_checkbox(window, "Agree", 20, 20))
gui_set_checked(checkbox, true)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some("err(string) for an unknown handle, or a handle that isn't a checkbox"),
    see_also: &["gui_is_checked", "gui_on_change", "gui_checkbox"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
