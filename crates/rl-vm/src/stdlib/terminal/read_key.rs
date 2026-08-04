use crate::stdlib::common::{verr, vok, vs};
use crate::{Vm, values::VmValue};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind, read};
use std::rc::Rc;

pub fn func(_: &mut Vm) -> VmValue {
    loop {
        match read() {
            Ok(Event::Key(key)) => {
                let s: String = match key.code {
                    KeyCode::Char(c) => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            format!("Ctrl:{c}")
                        } else {
                            format!("Char:{c}")
                        }
                    }
                    KeyCode::Enter => "Enter".into(),
                    KeyCode::Esc => "Esc".into(),
                    KeyCode::Backspace => "Backspace".into(),
                    KeyCode::Delete => "Delete".into(),
                    KeyCode::Tab => "Tab".into(),
                    KeyCode::BackTab => "BackTab".into(),
                    KeyCode::Up => "Up".into(),
                    KeyCode::Down => "Down".into(),
                    KeyCode::Left => "Left".into(),
                    KeyCode::Right => "Right".into(),
                    KeyCode::Home => "Home".into(),
                    KeyCode::End => "End".into(),
                    KeyCode::PageUp => "PageUp".into(),
                    KeyCode::PageDown => "PageDown".into(),
                    KeyCode::Insert => "Inseeval".into(),
                    KeyCode::F(n) => format!("F{n}"),
                    KeyCode::Null => "Null".into(),
                    _ => "Unknown".into(),
                };
                return vok!(VmValue::Arr(Rc::new(vec![vs!(s)],)));
            }

            // mouse events
            Ok(Event::Mouse(m)) => {
                let kind: String = match m.kind {
                    MouseEventKind::Down(MouseButton::Left) => "MouseLeft".into(),
                    MouseEventKind::Down(MouseButton::Right) => "MouseRight".into(),
                    MouseEventKind::Down(MouseButton::Middle) => "MouseMiddle".into(),
                    MouseEventKind::Up(_) => "MouseUp".into(),
                    MouseEventKind::Drag(_) => "MouseDrag".into(),
                    MouseEventKind::Moved => "MouseMove".into(),
                    MouseEventKind::ScrollUp => "ScrollUp".into(),
                    MouseEventKind::ScrollDown => "ScrollDown".into(),
                    _ => "MouseUnknown".into(),
                };
                return vok!(VmValue::Arr(Rc::new(vec![
                    vs!(kind),
                    vs!(m.column.to_string()),
                    vs!(m.row.to_string()),
                ],)));
            }
            // other events
            Ok(Event::Resize(cols, rows)) => {
                return vok!(VmValue::Arr(Rc::new(vec![
                    vs!("Resize".to_string()),
                    vs!(cols.to_string()),
                    vs!(rows.to_string()),
                ],)));
            }
            Ok(Event::FocusGained) => {
                return vok!(VmValue::Arr(Rc::new(vec![vs!("FocusGained".to_string())],)));
            }
            Ok(Event::FocusLost) => {
                return vok!(VmValue::Arr(Rc::new(vec![vs!("FocusLost".to_string())],)));
            }

            Err(e) => return verr!(vs!(format!("term_read_key(): {}", e))),
            _ => continue,
        }
    }
}
