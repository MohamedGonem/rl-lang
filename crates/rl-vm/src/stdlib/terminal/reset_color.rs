use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{execute, style::ResetColor};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_reset_color", execute!(stdout(), ResetColor));
    vok!(vnl!())
}
