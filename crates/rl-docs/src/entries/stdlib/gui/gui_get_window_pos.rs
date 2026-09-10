use crate::entry::FnEntry;

pub static GUI_GET_WINDOW_POS: FnEntry = FnEntry {
    signature: "gui_get_window_pos(window)",
    description: "returns the window position as [x, y] in logical pixels from the top-left corner",
    example: r#"get std::gui::gui_get_window_pos

dec handle win = gui_window("App", 800, 600)?
dec arr[float] pos = result_unwrap(gui_get_window_pos(win))?"#,
    expected_output: Some("[0.0, 0.0]"),
    returns: "result[array[float]]",
    errors: Some("Returns err if the handle is not a window"),
    see_also: &["gui_window", "gui_window_set_pos", "gui_get_window_size"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
