use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{cursor::RestorePosition, execute};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_restore_cursor", execute!(stdout(), RestorePosition));
    vok!(vnl!())
}
