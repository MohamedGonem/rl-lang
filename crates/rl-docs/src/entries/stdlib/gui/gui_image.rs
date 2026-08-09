use crate::entry::FnEntry;

pub static GUI_IMAGE: FnEntry = FnEntry {
    signature: "gui_image(window, width, height, rgba, x, y)",
    description: "adds a bitmap to `window` from raw RGBA8 pixel data, displayed at `width` x `height` pixels, positioned at `(x, y)`, and returns a handle to it. `rgba` must be an array[int] of exactly width * height * 4 bytes (0-255 each), laid out row-major top-to-bottom - the same representation `gui_window_set_icon` uses. std::gui has no image-decoding dependency, so callers must supply already-decoded pixel data. The image is re-uploaded to the GPU every frame rather than cached, so very large or frequently-changing images will cost more per frame than static widgets. Purely visual - no interaction, no `gui_on_change` - but supports `gui_set_visible`, `gui_is_visible`, `gui_set_pos`, `gui_get_pos`, and `gui_remove` like other widgets",
    example: r#"get std::gui::gui_window
get std::gui::gui_image

dec handle window = result_unwrap(gui_window("My App", 400, 300))

// a solid 2x2 red square, displayed at 64x64
dec array[int] pixels = [
    255, 0, 0, 255,   255, 0, 0, 255,
    255, 0, 0, 255,   255, 0, 0, 255,
]
dec handle logo = result_unwrap(gui_image(window, 64, 64, pixels, 20, 20))"#,
    expected_output: None,
    returns: "result[handle]",
    errors: Some(
        "err(string) if `window` is an unknown handle, is a handle that isn't a window, non-positive width/height, a byte outside 0-255, or an rgba array whose length doesn't match width * height * 4",
    ),
    see_also: &[
        "gui_window_set_icon",
        "gui_set_visible",
        "gui_set_pos",
        "gui_remove",
    ],
    since: Some("v0.4.0"),
};
