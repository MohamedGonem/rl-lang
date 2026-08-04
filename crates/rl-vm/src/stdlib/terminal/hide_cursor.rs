use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{cursor::Hide, execute};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_hide_cursor", execute!(stdout(), Hide));

    vok!(vnl!())
}
