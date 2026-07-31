//! `std::gui` - a native desktop GUI module built on `eframe`.

use crate::native::Module;
/// A single native GUI resource.
pub enum GuiHandle {
    Window(WindowState),
pub struct WindowState {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub visible: bool,
    pub children: Vec<u64>,
}

pub fn module() -> Module {
    Module::new("gui")
}
