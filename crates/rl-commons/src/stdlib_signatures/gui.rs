//! Typed signatures for `std::gui`.

use super::{fixed, handle, overloads, params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::{HandleKind, TypeAnnotation as T};

pub fn module() -> ModuleNames {
    ModuleNames::new("gui")
        .with_typed_function(gui_window())
        .with_typed_function(gui_button())
        .with_typed_function(gui_label())
        .with_typed_function(gui_checkbox())
        .with_typed_function(gui_textbox())
        .with_typed_function(gui_dropdown())
        .with_typed_function(gui_radio_group())
        .with_typed_function(gui_slider())
        .with_typed_function(gui_progress_bar())
        .with_typed_function(gui_set_text())
        .with_typed_function(gui_get_text())
        .with_typed_function(gui_set_visible())
        .with_typed_function(gui_is_visible())
        .with_typed_function(gui_on_click())
        .with_typed_function(gui_on_change())
        .with_typed_function(gui_is_checked())
        .with_typed_function(gui_set_checked())
        .with_typed_function(gui_get_selected_index())
        .with_typed_function(gui_set_selected_index())
        .with_typed_function(gui_get_selected())
        .with_typed_function(gui_get_value())
        .with_typed_function(gui_set_value())
        .with_typed_function(gui_set_progress())
        .with_typed_function(gui_get_progress())
        .with_typed_function(gui_set_pos())
        .with_typed_function(gui_get_pos())
        .with_typed_function(gui_remove())
        .with_typed_function(gui_window_set_title())
        .with_typed_function(gui_window_set_background())
        .with_typed_function(gui_window_set_size())
        .with_typed_function(gui_run())
        .with_typed_function(gui_close())
        .with_typed_function(gui_quit())
}

fn gui_window() -> StdFn {
    StdFn::typed(
        "gui_window",
        vec![(
            params(vec![T::String, T::Int, T::Int]),
            result(T::Handle(HandleKind::Gui)),
        )],
    )
}

fn gui_button() -> StdFn {
    StdFn::typed(
        "gui_button",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::String),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_label() -> StdFn {
    StdFn::typed(
        "gui_label",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::String),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_checkbox() -> StdFn {
    StdFn::typed(
        "gui_checkbox",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::String),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_textbox() -> StdFn {
    StdFn::typed(
        "gui_textbox",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::String),
                fixed(T::Int),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_dropdown() -> StdFn {
    StdFn::typed(
        "gui_dropdown",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::Array(Box::new(T::String))),
                fixed(T::Int),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_radio_group() -> StdFn {
    StdFn::typed(
        "gui_radio_group",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::Array(Box::new(T::String))),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_slider() -> StdFn {
    StdFn::typed(
        "gui_slider",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::Float),
                fixed(T::Float),
                fixed(T::Int),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_progress_bar() -> StdFn {
    StdFn::typed(
        "gui_progress_bar",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::Int),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Handle(HandleKind::Gui)),
        ),
    )
}

fn gui_set_text() -> StdFn {
    StdFn::typed(
        "gui_set_text",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::String)],
            result(T::Null),
        ),
    )
}

fn gui_get_text() -> StdFn {
    StdFn::typed(
        "gui_get_text",
        overloads(vec![handle(HandleKind::Gui)], result(T::String)),
    )
}

fn gui_set_visible() -> StdFn {
    StdFn::typed(
        "gui_set_visible",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Bool)],
            result(T::Null),
        ),
    )
}

fn gui_is_visible() -> StdFn {
    StdFn::typed(
        "gui_is_visible",
        overloads(vec![handle(HandleKind::Gui)], result(T::Bool)),
    )
}

fn gui_on_click() -> StdFn {
    StdFn::typed(
        "gui_on_click",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::Callback(vec![], Box::new(T::Null))),
            ],
            result(T::Null),
        ),
    )
}

fn gui_on_change() -> StdFn {
    StdFn::typed(
        "gui_on_change",
        vec![
            (
                params(vec![
                    T::Handle(HandleKind::Gui),
                    T::Callback(vec![T::Bool], Box::new(T::Null)),
                ]),
                result(T::Null),
            ),
            (
                params(vec![
                    T::Handle(HandleKind::Gui),
                    T::Callback(vec![T::Int], Box::new(T::Null)),
                ]),
                result(T::Null),
            ),
            (
                params(vec![
                    T::Handle(HandleKind::Gui),
                    T::Callback(vec![T::Float], Box::new(T::Null)),
                ]),
                result(T::Null),
            ),
        ],
    )
}

fn gui_is_checked() -> StdFn {
    StdFn::typed(
        "gui_is_checked",
        overloads(vec![handle(HandleKind::Gui)], result(T::Bool)),
    )
}

fn gui_set_checked() -> StdFn {
    StdFn::typed(
        "gui_set_checked",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Bool)],
            result(T::Null),
        ),
    )
}

fn gui_get_selected_index() -> StdFn {
    StdFn::typed(
        "gui_get_selected_index",
        overloads(vec![handle(HandleKind::Gui)], result(T::Int)),
    )
}

fn gui_set_selected_index() -> StdFn {
    StdFn::typed(
        "gui_set_selected_index",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Int)],
            result(T::Null),
        ),
    )
}

fn gui_get_selected() -> StdFn {
    StdFn::typed(
        "gui_get_selected",
        overloads(vec![handle(HandleKind::Gui)], result(T::String)),
    )
}

fn gui_get_value() -> StdFn {
    StdFn::typed(
        "gui_get_value",
        overloads(vec![handle(HandleKind::Gui)], result(T::Float)),
    )
}

fn gui_set_value() -> StdFn {
    StdFn::typed(
        "gui_set_value",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Float)],
            result(T::Null),
        ),
    )
}

fn gui_set_progress() -> StdFn {
    StdFn::typed(
        "gui_set_progress",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Float)],
            result(T::Null),
        ),
    )
}

fn gui_get_progress() -> StdFn {
    StdFn::typed(
        "gui_get_progress",
        overloads(vec![handle(HandleKind::Gui)], result(T::Float)),
    )
}

fn gui_set_pos() -> StdFn {
    StdFn::typed(
        "gui_set_pos",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Int), fixed(T::Int)],
            result(T::Null),
        ),
    )
}

fn gui_get_pos() -> StdFn {
    StdFn::typed(
        "gui_get_pos",
        overloads(
            vec![handle(HandleKind::Gui)],
            result(T::Tuple(std::rc::Rc::new(vec![T::Int, T::Int]))),
        ),
    )
}

fn gui_remove() -> StdFn {
    StdFn::typed(
        "gui_remove",
        overloads(vec![handle(HandleKind::Gui)], result(T::Null)),
    )
}

fn gui_window_set_title() -> StdFn {
    StdFn::typed(
        "gui_window_set_title",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::String)],
            result(T::Null),
        ),
    )
}

fn gui_window_set_background() -> StdFn {
    StdFn::typed(
        "gui_window_set_background",
        overloads(
            vec![
                handle(HandleKind::Gui),
                fixed(T::Int),
                fixed(T::Int),
                fixed(T::Int),
            ],
            result(T::Null),
        ),
    )
}

fn gui_window_set_size() -> StdFn {
    StdFn::typed(
        "gui_window_set_size",
        overloads(
            vec![handle(HandleKind::Gui), fixed(T::Int), fixed(T::Int)],
            result(T::Null),
        ),
    )
}

fn gui_run() -> StdFn {
    StdFn::typed(
        "gui_run",
        overloads(vec![handle(HandleKind::Gui)], result(T::Null)),
    )
}

fn gui_close() -> StdFn {
    StdFn::typed(
        "gui_close",
        overloads(vec![handle(HandleKind::Gui)], result(T::Null)),
    )
}

fn gui_quit() -> StdFn {
    StdFn::typed("gui_quit", vec![(params(vec![]), T::Null)])
}
