//! `std::gui` - a native desktop GUI module built on `eframe`.

use crate::native::Module;
use crate::values::Value;

mod common;
mod gui_button;
mod gui_checkbox;
mod gui_dropdown;
mod gui_label;
mod gui_radio_group;
mod gui_slider;
mod gui_textbox;
mod gui_window;

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

pub fn module() -> Module {
    Module::new("gui")
        .with_function("gui_window", gui_window::func)
        .with_function("gui_button", gui_button::func)
        .with_function("gui_label", gui_label::func)
        .with_function("gui_checkbox", gui_checkbox::func)
        .with_function("gui_textbox", gui_textbox::func)
        .with_function("gui_dropdown", gui_dropdown::func)
        .with_function("gui_radio_group", gui_radio_group::func)
}
