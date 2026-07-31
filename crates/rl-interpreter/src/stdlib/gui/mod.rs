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
mod gui_is_checked;
mod gui_is_visible;
mod gui_label;
mod gui_on_change;
mod gui_on_click;
mod gui_progress_bar;
mod gui_quit;
mod gui_radio_group;
mod gui_remove;
mod gui_run;
mod gui_set_checked;
mod gui_set_pos;
mod gui_set_progress;
mod gui_set_selected_index;
mod gui_set_text;
mod gui_set_value;
mod gui_set_visible;
mod gui_slider;
mod gui_textbox;
mod gui_window;
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
}

pub struct WindowState {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub children: Vec<u64>,
}

pub struct ButtonState {
    pub window: u64,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub on_click: Option<Value>,
}

pub struct LabelState {
    pub window: u64,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
}

pub struct CheckboxState {
    pub window: u64,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub visible: bool,
    pub checked: bool,
    pub on_change: Option<Value>,
}

pub struct TextboxState {
    pub window: u64,
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
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
}

pub struct ProgressState {
    pub window: u64,
    pub value: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub visible: bool,
}

pub fn module() -> Module {
    Module::new("gui")
        .with_function("gui_window", gui_window::func)
        .with_function("gui_button", gui_button::func)
        .with_function("gui_label", gui_label::func)
        .with_function("gui_checkbox", gui_checkbox::func)
        .with_function("gui_textbox", gui_textbox::func)
        .with_function("gui_dropdown", gui_dropdown::func)
        .with_function("gui_radio_group", gui_radio_group::func)
        .with_function("gui_slider", gui_slider::func)
        .with_function("gui_progress_bar", gui_progress_bar::func)
        .with_function("gui_set_text", gui_set_text::func)
        .with_function("gui_get_text", gui_get_text::func)
        .with_function("gui_set_visible", gui_set_visible::func)
        .with_function("gui_is_visible", gui_is_visible::func)
        .with_function("gui_on_click", gui_on_click::func)
        .with_function("gui_on_change", gui_on_change::func)
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
        .with_function("gui_remove", gui_remove::func)
        .with_function("gui_window_set_title", gui_window_set_title::func)
        .with_function("gui_run", gui_run::func)
        .with_function("gui_close", gui_close::func)
        .with_function("gui_quit", gui_quit::func)
}
