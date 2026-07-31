//! `std::gui` - a native desktop GUI module built on `eframe`.

use crate::native::Module;
use crate::values::Value;

mod common;
mod gui_button;
mod gui_checkbox;
mod gui_label;
mod gui_window;

pub use rl_commons::keywords::gui::KEYWORDS;

/// A single native GUI resource.
pub enum GuiHandle {
    Window(WindowState),
    Button(ButtonState),
    Label(LabelState),
    Checkbox(CheckboxState),
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

pub fn module() -> Module {
    Module::new("gui")
        .with_function("gui_window", gui_window::func)
}
