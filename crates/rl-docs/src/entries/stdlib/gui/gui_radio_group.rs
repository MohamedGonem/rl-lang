use crate::entry::FnEntry;

pub static GUI_RADIO_GROUP: FnEntry = FnEntry {
    signature: "gui_radio_group(window, options, x, y)",
    description: "adds a vertically-stacked group of radio buttons to `window`, one per entry in `options` (an `array[string]`, which must not be empty), anchored at `(x, y)`, and returns a handle to it. Unlike `gui_dropdown` there's no `width` parameter - each option's row sizes to its own label. Starts with index `0` selected; `gui_get_selected_index`/`gui_get_selected`/`gui_set_selected_index`/`gui_on_change` all work identically to their `gui_dropdown` counterparts, since both share the same underlying selection state",
    example: r#"get std::gui::gui_window
get std::gui::gui_radio_group

dec handle window = result_unwrap(gui_window("My App", 400, 300))
dec handle radios = result_unwrap(
    gui_radio_group(window, ["Option A", "Option B", "Option C"], 20, 20)
)"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle or isn't a window, if `options` isn't an `array[string]`, or if `options` is empty",
    ),
    see_also: &[
        "gui_dropdown",
        "gui_get_selected",
        "gui_get_selected_index",
        "gui_on_change",
    ],
    since: Some("v0.4.0"),
};
