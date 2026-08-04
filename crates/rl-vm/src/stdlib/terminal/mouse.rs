use crate::stdlib::macros::{try_fn, verr, vnl, vok, vs};
use crate::{Vm, values::VmValue};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use std::io::stdout;

pub fn std_term_enable_mouse(_: &mut Vm) -> VmValue {
    try_fn!("term_enable_mouse", execute!(stdout(), EnableMouseCapture));
    vok!(vnl!())
}

pub fn std_term_disable_mouse(_: &mut Vm) -> VmValue {
    try_fn!(
        "term_disable_mouse",
        execute!(stdout(), DisableMouseCapture)
    );
    vok!(vnl!())
}
