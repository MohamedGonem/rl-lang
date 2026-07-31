use rl_ast::statements::HandleKind;
use rl_utils::errors::Error;
use rl_utils::span::Span;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        common::{extract_handle, verr, vnl, vok, vs},
        gui::GuiHandle,
    },
    values::Value,
};

enum WidgetSnapshot {
    Button {
        id: u64,
        label: String,
        x: f32,
        y: f32,
    },
    Label {
        id: u64,
        text: String,
        x: f32,
        y: f32,
    },
    Checkbox {
        id: u64,
        label: String,
        x: f32,
        y: f32,
        checked: bool,
    },
    Textbox {
        id: u64,
        text: String,
        x: f32,
        y: f32,
        width: f32,
    },
    Dropdown {
        id: u64,
        options: Vec<String>,
        selected: usize,
        x: f32,
        y: f32,
        width: f32,
    },
    RadioGroup {
        id: u64,
        options: Vec<String>,
        selected: usize,
        x: f32,
        y: f32,
    },
    Slider {
        id: u64,
        value: f64,
        min: f64,
        max: f64,
        x: f32,
        y: f32,
        width: f32,
    },
    ProgressBar {
        id: u64,
        value: f32,
        x: f32,
        y: f32,
        width: f32,
    },
}

struct RlGuiApp<'a> {
    eval: &'a mut Evaluator,
    window: u64,
}

impl eframe::App for RlGuiApp<'_> {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        use eframe::egui;

        let ctx = ui.ctx().clone();
        let eval = &mut *self.eval;

        let Some(GuiHandle::Window(win)) = eval.gui_handles.get(&self.window) else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        };

        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(win.visible));
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(win.title.clone()));

        let background = win.background;
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(
                background.0,
                background.1,
                background.2,
            )))
            .show(ui, |_ui| {});

        let children = win.children.clone();
        let snapshots: Vec<WidgetSnapshot> = children
            .iter()
            .filter_map(|id| match eval.gui_handles.get(id) {
                Some(GuiHandle::Button(b)) if b.visible => Some(WidgetSnapshot::Button {
                    id: *id,
                    label: b.label.clone(),
                    x: b.x,
                    y: b.y,
                }),
                Some(GuiHandle::Label(l)) if l.visible => Some(WidgetSnapshot::Label {
                    id: *id,
                    text: l.text.clone(),
                    x: l.x,
                    y: l.y,
                }),
                Some(GuiHandle::Checkbox(c)) if c.visible => Some(WidgetSnapshot::Checkbox {
                    id: *id,
                    label: c.label.clone(),
                    x: c.x,
                    y: c.y,
                    checked: c.checked,
                }),
                Some(GuiHandle::Textbox(t)) if t.visible => Some(WidgetSnapshot::Textbox {
                    id: *id,
                    text: t.text.clone(),
                    x: t.x,
                    y: t.y,
                    width: t.width,
                }),
                Some(GuiHandle::Dropdown(s)) if s.visible => Some(WidgetSnapshot::Dropdown {
                    id: *id,
                    options: s.options.clone(),
                    selected: s.selected,
                    x: s.x,
                    y: s.y,
                    width: s.width,
                }),
                Some(GuiHandle::RadioGroup(s)) if s.visible => Some(WidgetSnapshot::RadioGroup {
                    id: *id,
                    options: s.options.clone(),
                    selected: s.selected,
                    x: s.x,
                    y: s.y,
                }),
                Some(GuiHandle::Slider(s)) if s.visible => Some(WidgetSnapshot::Slider {
                    id: *id,
                    value: s.value,
                    min: s.min,
                    max: s.max,
                    x: s.x,
                    y: s.y,
                    width: s.width,
                }),
                Some(GuiHandle::ProgressBar(p)) if p.visible => Some(WidgetSnapshot::ProgressBar {
                    id: *id,
                    value: p.value,
                    x: p.x,
                    y: p.y,
                    width: p.width,
                }),
                _ => None,
            })
            .collect();

        let mut clicked: Vec<u64> = Vec::new();
        let mut changed_checkbox: Vec<(u64, bool)> = Vec::new();
        let mut changed_text: Vec<(u64, String)> = Vec::new();
        let mut changed_selection: Vec<(u64, usize)> = Vec::new();
        let mut changed_value: Vec<(u64, f64)> = Vec::new();

        fn report_callback_err(result: Result<Value, Error>) {
            if let Err(e) = result {
                e.report_to_stderr();
            }
        }

        for snap in &snapshots {
            match snap {
                WidgetSnapshot::Button { id, label, x, y } => {
                    let resp = egui::Area::new(egui::Id::new(("rl_gui_button", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| ui.button(label))
                        .inner;
                    if resp.clicked() {
                        clicked.push(*id);
                    }
                }
                WidgetSnapshot::Label { id, text, x, y } => {
                    egui::Area::new(egui::Id::new(("rl_gui_label", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| ui.label(text));
                }
                WidgetSnapshot::Checkbox {
                    id,
                    label,
                    x,
                    y,
                    checked,
                } => {
                    let mut checked = *checked;
                    let resp = egui::Area::new(egui::Id::new(("rl_gui_checkbox", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| ui.checkbox(&mut checked, label))
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
                } => {
                    let mut text = text.clone();
                    let resp = egui::Area::new(egui::Id::new(("rl_gui_textbox", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| {
                            ui.add_sized([*width, 20.0], egui::TextEdit::singleline(&mut text))
                        })
                        .inner;
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
                } => {
                    let mut sel = *selected;
                    egui::Area::new(egui::Id::new(("rl_gui_dropdown", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| {
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
                } => {
                    let mut sel = *selected;
                    egui::Area::new(egui::Id::new(("rl_gui_radio", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| {
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
                } => {
                    let mut v = *value;
                    let resp = egui::Area::new(egui::Id::new(("rl_gui_slider", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| {
                            ui.add_sized(
                                [*width, 20.0],
                                egui::Slider::new(&mut v, *min..=*max).show_value(true),
                            )
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
                } => {
                    egui::Area::new(egui::Id::new(("rl_gui_progress", *id)))
                        .fixed_pos(egui::pos2(*x, *y))
                        .show(&ctx, |ui| {
                            ui.add_sized([*width, 20.0], egui::ProgressBar::new(*value))
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

        if eval.gui_quit_requested {
            eval.gui_quit_requested = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        ctx.request_repaint();
    }
}

pub fn func(eval: &mut Evaluator, window: Value) -> Value {
    let window_id = match extract_handle(window, HandleKind::Gui, "gui_run") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    let (title, width, height) = match eval.gui_handles.get(&window_id) {
        Some(GuiHandle::Window(w)) => (w.title.clone(), w.width, w.height),
        Some(_) => {
            return verr!(vs!(format!(
                "gui_run: handle {} is not a window",
                window_id
            )));
        }
        None => return verr!(vs!(format!("gui_run: unknown handle {}", window_id))),
    };

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title(title.clone())
            .with_inner_size([width, height]),
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
