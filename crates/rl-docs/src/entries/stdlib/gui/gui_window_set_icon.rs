use crate::entry::FnEntry;

pub static GUI_WINDOW_SET_ICON: FnEntry = FnEntry {
    signature: "gui_window_set_icon(window, width, height, rgba)",
    description: "sets `window`'s icon from raw RGBA8 pixel data. `rgba` must be an array[int] of exactly width * height * 4 bytes (0-255 each), laid out row-major top-to-bottom. std::gui has no image-decoding dependency, so callers must supply already-decoded pixel data - e.g. a small precomputed icon, or bytes decoded by another library. Takes effect on the next gui_run frame if the window is already open",
    example: r#"get std::gui::gui_window
get std::gui::gui_window_set_icon

dec handle window = result_unwrap(gui_window("My App", 400, 300))

// a solid 2x2 red icon
dec array[int] pixels = [
    255, 0, 0, 255,   255, 0, 0, 255,
    255, 0, 0, 255,   255, 0, 0, 255,
]
gui_window_set_icon(window, 2, 2, pixels)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "err(string) for an unknown handle, a handle that isn't a window, non-positive width/height, a byte outside 0-255, or an rgba array whose length doesn't match width * height * 4",
    ),
    see_also: &["gui_window", "gui_window_set_decorated", "gui_run"],
    since: Some("v0.4.0"),
};
