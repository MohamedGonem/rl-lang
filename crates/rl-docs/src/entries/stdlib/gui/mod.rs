use crate::entry::{FnEntry, StdEntry};

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
mod gui_window_set_background;
mod gui_window_set_title;

pub static GUI: StdEntry = StdEntry {
    name: "gui",
    description: "a native desktop GUI toolkit (egui/eframe) for building windows out of absolutely-positioned widgets - buttons, labels, checkboxes, textboxes, dropdowns, radio groups, sliders, and progress bars",
    functions: FUNCTIONS,
    since: Some("v0.4.0"),
    unstable: true,
};

// Widget functions: create a widget and return a handle to it.
// Control functions: get/set a widget's state.
// Event functions: attach callbacks.
// Lifecycle functions: open/close the native window and its event loop.
static FUNCTIONS: &[&FnEntry] = &[
    // widget functions
    &gui_window::GUI_WINDOW,
    &gui_button::GUI_BUTTON,
    &gui_label::GUI_LABEL,
    &gui_checkbox::GUI_CHECKBOX,
    &gui_textbox::GUI_TEXTBOX,
    &gui_dropdown::GUI_DROPDOWN,
    &gui_radio_group::GUI_RADIO_GROUP,
    &gui_slider::GUI_SLIDER,
    &gui_progress_bar::GUI_PROGRESS_BAR,
    // control functions
    &gui_set_text::GUI_SET_TEXT,
    &gui_get_text::GUI_GET_TEXT,
    &gui_set_visible::GUI_SET_VISIBLE,
    &gui_is_visible::GUI_IS_VISIBLE,
    &gui_is_checked::GUI_IS_CHECKED,
    &gui_set_checked::GUI_SET_CHECKED,
    &gui_get_selected_index::GUI_GET_SELECTED_INDEX,
    &gui_set_selected_index::GUI_SET_SELECTED_INDEX,
    &gui_get_selected::GUI_GET_SELECTED,
    &gui_get_value::GUI_GET_VALUE,
    &gui_set_value::GUI_SET_VALUE,
    &gui_set_progress::GUI_SET_PROGRESS,
    &gui_get_progress::GUI_GET_PROGRESS,
    &gui_set_pos::GUI_SET_POS,
    &gui_get_pos::GUI_GET_POS,
    &gui_remove::GUI_REMOVE,
    &gui_window_set_title::GUI_WINDOW_SET_TITLE,
    &gui_window_set_background::GUI_WINDOW_SET_BACKGROUND,
    // event functions
    &gui_on_click::GUI_ON_CLICK,
    &gui_on_change::GUI_ON_CHANGE,
    // lifecycle functions
    &gui_run::GUI_RUN,
    &gui_close::GUI_CLOSE,
    &gui_quit::GUI_QUIT,
];
