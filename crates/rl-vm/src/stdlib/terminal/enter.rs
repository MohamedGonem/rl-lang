use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_enter", enable_raw_mode());
    try_fn!("term_enter", execute!(stdout(), EnterAlternateScreen));

    vok!(vnl!())
}
