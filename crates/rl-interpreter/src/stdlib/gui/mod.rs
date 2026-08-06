//! `std::gui` - a native desktop GUI module built on `eframe`.

use crate::native::Module;
use crate::values::Value;

mod common;
mod gui_button;
mod gui_checkbox;
mod gui_close;
mod gui_dropdown;
mod gui_get_pos;
mod gui_get_progress;
mod gui_get_selected;
mod gui_get_selected_index;
mod gui_get_text;
mod gui_get_value;
mod gui_get_z;
mod gui_image;
mod gui_is_checked;
mod gui_is_visible;
mod gui_label;
mod gui_number_input;
mod gui_on_change;
mod gui_on_click;
mod gui_on_close;
mod gui_on_key;
mod gui_on_submit;
mod gui_progress_bar;
mod gui_quit;
mod gui_radio_group;
mod gui_remove;
mod gui_run;
mod gui_separator;
mod gui_set_checked;
mod gui_set_pos;
mod gui_set_progress;
mod gui_set_selected_index;
mod gui_set_text;
mod gui_set_value;
mod gui_set_visible;
mod gui_set_z;
mod gui_slider;
mod gui_textarea;
mod gui_textbox;
mod gui_window;
mod gui_window_set_background;
mod gui_window_set_decorated;
mod gui_window_set_icon;
mod gui_window_set_pos;
mod gui_window_set_size;
mod gui_window_set_title;

pub use rl_commons::keywords::gui::KEYWORDS;

/// A single native GUI resource.
pub enum GuiHandle {
    Window(WindowState),
    Button(ButtonState),
    Label(LabelState),
    Checkbox(CheckboxState),
    Textbox(TextboxState),
    Dropdown(SelectState),
    RadioGroup(SelectState),
    Slider(SliderState),
    ProgressBar(ProgressState),
    Separator(SeparatorState),
    Image(ImageState),
}

pub struct WindowState {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub children: Vec<u64>,
    /// RGB background fill for the window's central panel.
    pub background: (u8, u8, u8),
    /// A pending resize request from `gui_window_set_size`, applied once and
    /// then cleared so it doesn't fight the user manually resizing the
    /// window afterward.
    pub pending_size: Option<(f32, f32)>,
    /// A pending move request from `gui_window_set_pos`, applied once and
    /// then cleared so it doesn't fight the user manually dragging the
    /// window afterward.
    pub pending_position: Option<(f32, f32)>,
    /// Whether the native title bar and window borders are shown.
    pub decorated: bool,
    /// Window icon as `(width, height, rgba bytes)`. `None` uses the OS default.
    pub icon: Option<(u32, u32, Vec<u8>)>,
    /// Called with no arguments when this window closes, whether via
    /// `gui_close` or the native close button.
    pub on_close: Option<Value>,
    /// Called with the pressed key's name (e.g. `"Enter"`, `"Escape"`) for
    /// every non-repeat key press while this window has focus.
    pub on_key: Option<Value>,
}

pub struct ButtonState {
    pub window: u64,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub on_click: Option<Value>,
    /// Draw order among this window's widgets: higher draws on top of
    /// lower when positions overlap. Widgets with equal z draw in creation
    /// order (later created = on top).
    pub z: i32,
}

pub struct LabelState {
    pub window: u64,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub z: i32,
}

pub struct CheckboxState {
    pub window: u64,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub checked: bool,
    pub on_change: Option<Value>,
    pub z: i32,
}

pub struct TextboxState {
    pub window: u64,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub on_change: Option<Value>,
    /// Called with the current text when Enter is pressed while this
    /// textbox has focus. Never fires when `multiline` is true, since Enter
    /// inserts a newline there instead of submitting.
    pub on_submit: Option<Value>,
    /// Whether this is a multiline textarea (`gui_textarea`) rather than a
    /// single-line textbox (`gui_textbox`). Both share this same state and
    /// every get/set/visibility/position/remove/on_change function.
    pub multiline: bool,
    /// Row height in points. Only meaningful when `multiline` is true -
    /// single-line textboxes use a fixed row height instead.
    pub height: f32,
    pub z: i32,
}

pub struct SelectState {
    pub window: u64,
    pub options: Vec<String>,
    pub selected: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub on_change: Option<Value>,
    pub z: i32,
}

pub struct SliderState {
    pub window: u64,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub on_change: Option<Value>,
    /// When true, renders as a compact draggable/typeable number field
    /// (`gui_number_input`) instead of a slider bar (`gui_slider`). Both
    /// share this same state and every get/set/visibility/position/remove/
    /// on_change function.
    pub drag_only: bool,
    pub z: i32,
}

pub struct ProgressState {
    pub window: u64,
    pub value: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub z: i32,
}

pub struct SeparatorState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
    pub z: i32,
}

pub struct ImageState {
    pub window: u64,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    /// Raw RGBA8 pixel data as `(width, height, bytes)`, row-major
    /// top-to-bottom - same representation as `gui_window_set_icon`, since
    /// `std::gui` has no image-decoding dependency. `Arc`-wrapped since this
    /// gets cloned every frame while rendering.
    pub rgba: (u32, u32, std::sync::Arc<Vec<u8>>),
    pub z: i32,
}

pub fn module() -> Module {
    Module::new("gui")
        .with_function("gui_window", gui_window::func)
        .with_function("gui_button", gui_button::func)
        .with_function("gui_label", gui_label::func)
        .with_function("gui_checkbox", gui_checkbox::func)
        .with_function("gui_textbox", gui_textbox::func)
        .with_function("gui_textarea", gui_textarea::func)
        .with_function("gui_dropdown", gui_dropdown::func)
        .with_function("gui_radio_group", gui_radio_group::func)
        .with_function("gui_slider", gui_slider::func)
        .with_function("gui_number_input", gui_number_input::func)
        .with_function("gui_progress_bar", gui_progress_bar::func)
        .with_function("gui_separator", gui_separator::func)
        .with_function("gui_image", gui_image::func)
        .with_function("gui_set_text", gui_set_text::func)
        .with_function("gui_get_text", gui_get_text::func)
        .with_function("gui_set_visible", gui_set_visible::func)
        .with_function("gui_is_visible", gui_is_visible::func)
        .with_function("gui_on_click", gui_on_click::func)
        .with_function("gui_on_change", gui_on_change::func)
        .with_function("gui_on_submit", gui_on_submit::func)
        .with_function("gui_on_key", gui_on_key::func)
        .with_function("gui_on_close", gui_on_close::func)
        .with_function("gui_is_checked", gui_is_checked::func)
        .with_function("gui_set_checked", gui_set_checked::func)
        .with_function("gui_get_selected_index", gui_get_selected_index::func)
        .with_function("gui_set_selected_index", gui_set_selected_index::func)
        .with_function("gui_get_selected", gui_get_selected::func)
        .with_function("gui_get_value", gui_get_value::func)
        .with_function("gui_set_value", gui_set_value::func)
        .with_function("gui_set_progress", gui_set_progress::func)
        .with_function("gui_get_progress", gui_get_progress::func)
        .with_function("gui_set_pos", gui_set_pos::func)
        .with_function("gui_get_pos", gui_get_pos::func)
        .with_function("gui_set_z", gui_set_z::func)
        .with_function("gui_get_z", gui_get_z::func)
        .with_function("gui_remove", gui_remove::func)
        .with_function("gui_window_set_title", gui_window_set_title::func)
        .with_function("gui_window_set_background", gui_window_set_background::func)
        .with_function("gui_window_set_size", gui_window_set_size::func)
        .with_function("gui_window_set_pos", gui_window_set_pos::func)
        .with_function("gui_window_set_decorated", gui_window_set_decorated::func)
        .with_function("gui_window_set_icon", gui_window_set_icon::func)
        .with_function("gui_run", gui_run::func)
        .with_function("gui_close", gui_close::func)
        .with_function("gui_quit", gui_quit::func)
}
