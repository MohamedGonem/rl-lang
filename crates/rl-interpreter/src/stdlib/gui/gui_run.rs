use eframe::egui;
use rl_ast::statements::HandleKind;
use rl_utils::span::Span;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::{
            GuiHandle,
            common::{close_window, report_callback_err},
        },
    },
    values::Value,
};

enum WidgetSnapshot {
    Button {
        id: u64,
        label: String,
        x: f32,
        y: f32,
        z: i32,
    },
    Label {
        id: u64,
        text: String,
        x: f32,
        y: f32,
        z: i32,
    },
    Checkbox {
        id: u64,
        label: String,
        x: f32,
        y: f32,
        checked: bool,
        z: i32,
    },
    Textbox {
        id: u64,
        text: String,
        x: f32,
        y: f32,
        width: f32,
        multiline: bool,
        height: f32,
        z: i32,
    },
    Dropdown {
        id: u64,
        options: Vec<String>,
        selected: usize,
        x: f32,
        y: f32,
        width: f32,
        z: i32,
    },
    RadioGroup {
        id: u64,
        options: Vec<String>,
        selected: usize,
        x: f32,
        y: f32,
        z: i32,
    },
    Slider {
        id: u64,
        value: f64,
        min: f64,
        max: f64,
        x: f32,
        y: f32,
        width: f32,
        drag_only: bool,
        z: i32,
    },
    ProgressBar {
        id: u64,
        value: f32,
        x: f32,
        y: f32,
        width: f32,
        z: i32,
    },
    Separator {
        id: u64,
        x: f32,
        y: f32,
        width: f32,
        z: i32,
    },
    Image {
        id: u64,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_width: u32,
        texture_height: u32,
        rgba: std::sync::Arc<Vec<u8>>,
        z: i32,
    },
}

impl WidgetSnapshot {
    /// Draw order among a window's widgets: higher draws on top. Used to
    /// stable-sort snapshots before rendering, so widgets with equal z keep
    /// drawing in creation order (the order `sort_by_key` preserves for
    /// equal keys).
    fn z(&self) -> i32 {
        match self {
            WidgetSnapshot::Button { z, .. }
            | WidgetSnapshot::Label { z, .. }
            | WidgetSnapshot::Checkbox { z, .. }
            | WidgetSnapshot::Textbox { z, .. }
            | WidgetSnapshot::Dropdown { z, .. }
            | WidgetSnapshot::RadioGroup { z, .. }
            | WidgetSnapshot::Slider { z, .. }
            | WidgetSnapshot::ProgressBar { z, .. }
            | WidgetSnapshot::Separator { z, .. }
            | WidgetSnapshot::Image { z, .. } => *z,
        }
    }
}

/// Renders one window's background and widgets for the current frame, and
/// dispatches any callbacks triggered by this frame's interactions.
///
/// `ctx` must be scoped to the viewport `window_id` corresponds to: the
/// root's own `Context` for the root window, or the `Context` handed to a
/// `show_viewport_immediate` closure for a secondary window. Every widget is
/// drawn as its own `egui::Area`, so this same code works unmodified for
/// either case - `Area::show` is always `Context`-based, unlike panels.
fn render_window(eval: &mut Evaluator, ctx: &egui::Context, window_id: u64) {
    let Some(GuiHandle::Window(win)) = eval.gui_handles.get(&window_id) else {
        return;
    };
    let background = win.background;
    let children = win.children.clone();
    let enter_pressed = ctx.input(|i| i.key_pressed(egui::Key::Enter));

    // Background fill: a full-viewport Area at `Order::Background`, painted
    // manually, rather than a panel. This sidesteps relying on the exact
    // panel API (which has shifted across egui versions) since `Area` is the
    // one drawing primitive every widget here already depends on.
    egui::Area::new(egui::Id::new(("rl_gui_window_bg", window_id)))
        .order(egui::Order::Background)
        .fixed_pos(egui::pos2(0.0, 0.0))
        .show(ctx, |ui| {
            let rect = ctx
                .input(|i| i.raw.screen_rect)
                .unwrap_or(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(4096.0, 4096.0),
                ));
            ui.painter().rect_filled(
                rect,
                0.0,
                egui::Color32::from_rgb(background.0, background.1, background.2),
            );
        });

    let mut snapshots: Vec<WidgetSnapshot> = children
        .iter()
        .filter_map(|id| match eval.gui_handles.get(id) {
            Some(GuiHandle::Button(b)) if b.visible => Some(WidgetSnapshot::Button {
                id: *id,
                label: b.label.clone(),
                x: b.x,
                y: b.y,
                z: b.z,
            }),
            Some(GuiHandle::Label(l)) if l.visible => Some(WidgetSnapshot::Label {
                id: *id,
                text: l.text.clone(),
                x: l.x,
                y: l.y,
                z: l.z,
            }),
            Some(GuiHandle::Checkbox(c)) if c.visible => Some(WidgetSnapshot::Checkbox {
                id: *id,
                label: c.label.clone(),
                x: c.x,
                y: c.y,
                checked: c.checked,
                z: c.z,
            }),
            Some(GuiHandle::Textbox(t)) if t.visible => Some(WidgetSnapshot::Textbox {
                id: *id,
                text: t.text.clone(),
                x: t.x,
                y: t.y,
                width: t.width,
                multiline: t.multiline,
                height: t.height,
                z: t.z,
            }),
            Some(GuiHandle::Dropdown(s)) if s.visible => Some(WidgetSnapshot::Dropdown {
                id: *id,
                options: s.options.clone(),
                selected: s.selected,
                x: s.x,
                y: s.y,
                width: s.width,
                z: s.z,
            }),
            Some(GuiHandle::RadioGroup(s)) if s.visible => Some(WidgetSnapshot::RadioGroup {
                id: *id,
                options: s.options.clone(),
                selected: s.selected,
                x: s.x,
                y: s.y,
                z: s.z,
            }),
            Some(GuiHandle::Slider(s)) if s.visible => Some(WidgetSnapshot::Slider {
                id: *id,
                value: s.value,
                min: s.min,
                max: s.max,
                x: s.x,
                y: s.y,
                width: s.width,
                drag_only: s.drag_only,
                z: s.z,
            }),
            Some(GuiHandle::ProgressBar(p)) if p.visible => Some(WidgetSnapshot::ProgressBar {
                id: *id,
                value: p.value,
                x: p.x,
                y: p.y,
                width: p.width,
                z: p.z,
            }),
            Some(GuiHandle::Separator(s)) if s.visible => Some(WidgetSnapshot::Separator {
                id: *id,
                x: s.x,
                y: s.y,
                width: s.width,
                z: s.z,
            }),
            Some(GuiHandle::Image(img)) if img.visible => Some(WidgetSnapshot::Image {
                id: *id,
                x: img.x,
                y: img.y,
                width: img.width,
                height: img.height,
                texture_width: img.rgba.0,
                texture_height: img.rgba.1,
                rgba: img.rgba.2.clone(),
                z: img.z,
            }),
            _ => None,
        })
        .collect();

    // Stable sort: widgets with equal z keep the relative order they were
    // already in (children's creation order), only differing z reorders
    // them. Higher z draws later, i.e. on top, since each widget is its own
    // Area drawn in this loop's order.
    snapshots.sort_by_key(WidgetSnapshot::z);

    let mut clicked: Vec<u64> = Vec::new();
    let mut changed_checkbox: Vec<(u64, bool)> = Vec::new();
    let mut changed_text: Vec<(u64, String)> = Vec::new();
    let mut submitted: Vec<(u64, String)> = Vec::new();
    let mut changed_selection: Vec<(u64, usize)> = Vec::new();
    let mut changed_value: Vec<(u64, f64)> = Vec::new();

    for snap in &snapshots {
        match snap {
            WidgetSnapshot::Button {
                id, label, x, y, ..
            } => {
                let resp = egui::Area::new(egui::Id::new(("rl_gui_button", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| ui.button(label))
                    .inner;
                if resp.clicked() {
                    clicked.push(*id);
                }
            }
            WidgetSnapshot::Label { id, text, x, y, .. } => {
                egui::Area::new(egui::Id::new(("rl_gui_label", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| ui.label(text));
            }
            WidgetSnapshot::Checkbox {
                id,
                label,
                x,
                y,
                checked,
                ..
            } => {
                let mut checked = *checked;
                let resp = egui::Area::new(egui::Id::new(("rl_gui_checkbox", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| ui.checkbox(&mut checked, label))
                    .inner;
                if resp.changed() {
                    changed_checkbox.push((*id, checked));
                }
            }
            WidgetSnapshot::Textbox {
                id,
                text,
                x,
                y,
                width,
                multiline,
                height,
                ..
            } => {
                let mut text = text.clone();
                let resp = egui::Area::new(egui::Id::new(("rl_gui_textbox", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        if *multiline {
                            ui.add_sized([*width, *height], egui::TextEdit::multiline(&mut text))
                        } else {
                            ui.add_sized([*width, 20.0], egui::TextEdit::singleline(&mut text))
                        }
                    })
                    .inner;
                if !multiline && resp.lost_focus() && enter_pressed {
                    submitted.push((*id, text.clone()));
                }
                if resp.changed() {
                    changed_text.push((*id, text));
                }
            }
            WidgetSnapshot::Dropdown {
                id,
                options,
                selected,
                x,
                y,
                width,
                ..
            } => {
                let mut sel = *selected;
                egui::Area::new(egui::Id::new(("rl_gui_dropdown", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        egui::ComboBox::from_id_salt(("rl_gui_dropdown_combo", *id))
                            .width(*width)
                            .selected_text(options.get(sel).cloned().unwrap_or_default())
                            .show_ui(ui, |ui| {
                                for (i, opt) in options.iter().enumerate() {
                                    ui.selectable_value(&mut sel, i, opt);
                                }
                            });
                    });
                if sel != *selected {
                    changed_selection.push((*id, sel));
                }
            }
            WidgetSnapshot::RadioGroup {
                id,
                options,
                selected,
                x,
                y,
                ..
            } => {
                let mut sel = *selected;
                egui::Area::new(egui::Id::new(("rl_gui_radio", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            for (i, opt) in options.iter().enumerate() {
                                ui.radio_value(&mut sel, i, opt);
                            }
                        });
                    });
                if sel != *selected {
                    changed_selection.push((*id, sel));
                }
            }
            WidgetSnapshot::Slider {
                id,
                value,
                min,
                max,
                x,
                y,
                width,
                drag_only,
                ..
            } => {
                let mut v = *value;
                let resp = egui::Area::new(egui::Id::new(("rl_gui_slider", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        if *drag_only {
                            ui.add(egui::DragValue::new(&mut v).range(*min..=*max))
                        } else {
                            ui.add_sized(
                                [*width, 20.0],
                                egui::Slider::new(&mut v, *min..=*max).show_value(true),
                            )
                        }
                    })
                    .inner;
                if resp.changed() {
                    changed_value.push((*id, v));
                }
            }
            WidgetSnapshot::ProgressBar {
                id,
                value,
                x,
                y,
                width,
                ..
            } => {
                egui::Area::new(egui::Id::new(("rl_gui_progress", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.add_sized([*width, 20.0], egui::ProgressBar::new(*value))
                    });
            }
            WidgetSnapshot::Separator {
                id, x, y, width, ..
            } => {
                egui::Area::new(egui::Id::new(("rl_gui_separator", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.allocate_ui(egui::vec2(*width, 6.0), |ui| {
                            ui.separator();
                        });
                    });
            }
            WidgetSnapshot::Image {
                id,
                x,
                y,
                width,
                height,
                texture_width,
                texture_height,
                rgba,
                ..
            } => {
                // Re-uploaded as a fresh texture every frame rather than
                // cached, since caching would need to track texture
                // lifetime against widget removal. Simple and correct;
                // costly for large or frequently-redrawn images.
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    [*texture_width as usize, *texture_height as usize],
                    rgba,
                );
                let texture = ctx.load_texture(
                    format!("rl_gui_image_{}", id),
                    color_image,
                    egui::TextureOptions::default(),
                );
                // SizedTexture's size is the *display* size, independent of
                // the texture's actual pixel dimensions - GPU sampling
                // stretches to fit, so this alone gives us "display at
                // exactly width x height" with no extra builder calls.
                let sized =
                    egui::load::SizedTexture::new(texture.id(), egui::vec2(*width, *height));
                egui::Area::new(egui::Id::new(("rl_gui_image", *id)))
                    .fixed_pos(egui::pos2(*x, *y))
                    .show(ctx, |ui| {
                        ui.add(egui::Image::from_texture(sized));
                    });
            }
        }
    }

    for (id, text) in changed_text {
        let callback = match eval.gui_handles.get_mut(&id) {
            Some(GuiHandle::Textbox(t)) => {
                t.text = text.clone();
                t.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(eval.call_value(cb, vec![Value::String(text)], Span::dummy()));
        }
    }

    for (id, text) in submitted {
        let callback = match eval.gui_handles.get(&id) {
            Some(GuiHandle::Textbox(t)) => t.on_submit.clone(),
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(eval.call_value(cb, vec![Value::String(text)], Span::dummy()));
        }
    }

    for (id, checked) in changed_checkbox {
        let callback = match eval.gui_handles.get_mut(&id) {
            Some(GuiHandle::Checkbox(c)) => {
                c.checked = checked;
                c.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(eval.call_value(cb, vec![Value::Bool(checked)], Span::dummy()));
        }
    }

    for (id, sel) in changed_selection {
        let callback = match eval.gui_handles.get_mut(&id) {
            Some(GuiHandle::Dropdown(s)) | Some(GuiHandle::RadioGroup(s)) => {
                s.selected = sel;
                s.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(eval.call_value(
                cb,
                vec![Value::Integer(sel as i64)],
                Span::dummy(),
            ));
        }
    }

    for (id, v) in changed_value {
        let callback = match eval.gui_handles.get_mut(&id) {
            Some(GuiHandle::Slider(s)) => {
                s.value = v;
                s.on_change.clone()
            }
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(eval.call_value(cb, vec![Value::Float(v)], Span::dummy()));
        }
    }

    for id in clicked {
        let callback = match eval.gui_handles.get(&id) {
            Some(GuiHandle::Button(b)) => b.on_click.clone(),
            _ => None,
        };
        if let Some(cb) = callback {
            report_callback_err(eval.call_value(cb, vec![], Span::dummy()));
        }
    }

    let key_names: Vec<String> = ctx.input(|i| {
        i.events
            .iter()
            .filter_map(|e| match e {
                egui::Event::Key {
                    key,
                    pressed: true,
                    repeat: false,
                    ..
                } => Some(format!("{:?}", key)),
                _ => None,
            })
            .collect()
    });

    if !key_names.is_empty() {
        let on_key = match eval.gui_handles.get(&window_id) {
            Some(GuiHandle::Window(w)) => w.on_key.clone(),
            _ => None,
        };
        if let Some(cb) = on_key {
            for key_name in key_names {
                report_callback_err(eval.call_value(
                    cb.clone(),
                    vec![Value::String(key_name)],
                    Span::dummy(),
                ));
            }
        }
    }
}

/// A window's viewport-level state (title/visible/decorated/icon/pending
/// size & position), snapshotted with the one-shot pending fields cleared.
/// Shared by the root window (driven via `ViewportCommand`s) and secondary
/// windows (driven via their `ViewportBuilder`, rebuilt every frame).
struct ViewportState {
    title: String,
    visible: bool,
    decorated: bool,
    icon: Option<(u32, u32, Vec<u8>)>,
    pending_size: Option<(f32, f32)>,
    pending_position: Option<(f32, f32)>,
}

fn take_viewport_state(eval: &mut Evaluator, window_id: u64) -> Option<ViewportState> {
    let Some(GuiHandle::Window(w)) = eval.gui_handles.get(&window_id) else {
        return None;
    };
    let state = ViewportState {
        title: w.title.clone(),
        visible: w.visible,
        decorated: w.decorated,
        icon: w.icon.clone(),
        pending_size: w.pending_size,
        pending_position: w.pending_position,
    };

    if (state.pending_size.is_some() || state.pending_position.is_some())
        && let Some(GuiHandle::Window(w)) = eval.gui_handles.get_mut(&window_id)
    {
        w.pending_size = None;
        w.pending_position = None;
    }

    Some(state)
}

struct RlGuiApp<'a> {
    eval: &'a mut Evaluator,
    window: u64,
}

impl eframe::App for RlGuiApp<'_> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let eval = &mut *self.eval;

        // If the user clicked the native close button, clean up (and fire
        // `on_close`) now. eframe still closes the native window at the end
        // of this frame regardless - this just makes sure the callback runs
        // and any child widget handles are freed rather than leaked.
        if ctx.input(|i| i.viewport().close_requested()) {
            close_window(eval, self.window);
        }

        let Some(state) = take_viewport_state(eval, self.window) else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(state.visible));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(state.title));
        ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(state.decorated));
        if let Some((width, height)) = state.pending_size {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(width, height)));
        }
        if let Some((x, y)) = state.pending_position {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(x, y)));
        }
        if let Some((icon_width, icon_height, rgba)) = state.icon {
            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(std::sync::Arc::new(
                egui::IconData {
                    rgba,
                    width: icon_width,
                    height: icon_height,
                },
            ))));
        }

        render_window(eval, &ctx, self.window);

        // Every other open window becomes its own native viewport, spawned
        // fresh each frame (immediate viewports must be re-requested every
        // frame they should stay visible - stop calling this for an id and
        // its window closes).
        let secondary_window_ids: Vec<u64> = eval
            .gui_handles
            .iter()
            .filter_map(|(id, handle)| match handle {
                GuiHandle::Window(_) if *id != self.window => Some(*id),
                _ => None,
            })
            .collect();

        for window_id in secondary_window_ids {
            let Some(state) = take_viewport_state(eval, window_id) else {
                continue;
            };

            let mut builder = egui::ViewportBuilder::default()
                .with_title(state.title)
                .with_decorations(state.decorated)
                .with_visible(state.visible);
            if let Some((width, height)) = state.pending_size {
                builder = builder.with_inner_size([width, height]);
            }
            if let Some((x, y)) = state.pending_position {
                builder = builder.with_position([x, y]);
            }
            if let Some((icon_width, icon_height, rgba)) = state.icon {
                builder = builder.with_icon(egui::IconData {
                    rgba,
                    width: icon_width,
                    height: icon_height,
                });
            }

            let viewport_id = egui::ViewportId::from_hash_of(("rl_gui_window", window_id));
            ctx.show_viewport_immediate(viewport_id, builder, |child_ctx, _class| {
                if child_ctx.input(|i| i.viewport().close_requested()) {
                    close_window(&mut *eval, window_id);
                    return;
                }
                render_window(&mut *eval, child_ctx, window_id);
            });
        }

        if eval.gui_quit_requested {
            eval.gui_quit_requested = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // ctx.request_repaint();
    }
}

pub fn func(eval: &mut Evaluator, window: Value) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_run") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let (title, width, height, position, decorated, icon) = match eval.gui_handles.get(&window_id) {
        Some(GuiHandle::Window(w)) => {
            let (width, height) = w.pending_size.unwrap_or((w.width, w.height));
            (
                w.title.clone(),
                width,
                height,
                w.pending_position,
                w.decorated,
                w.icon.clone(),
            )
        }
        Some(_) => {
            return verr!(vs!(format!(
                "gui_run: handle {} is not a window",
                window_id
            )));
        }
        None => return verr!(vs!(format!("gui_run: unknown handle {}", window_id))),
    };

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(title.clone())
        .with_inner_size([width, height])
        .with_decorations(decorated);
    if let Some((x, y)) = position {
        viewport = viewport.with_position([x, y]);
    }
    if let Some((icon_width, icon_height, rgba)) = icon {
        viewport = viewport.with_icon(egui::IconData {
            rgba,
            width: icon_width,
            height: icon_height,
        });
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let result = eframe::run_native(
        &title,
        options,
        Box::new(move |_cc| {
            Ok(Box::new(RlGuiApp {
                eval,
                window: window_id,
            }))
        }),
    );

    match result {
        Ok(()) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!("gui_run: {}", e))),
    }
}
