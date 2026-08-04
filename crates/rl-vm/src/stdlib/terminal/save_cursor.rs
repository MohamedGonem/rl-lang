use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{cursor::SavePosition, execute};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_save_cursor", execute!(stdout(), SavePosition));
    vok!(vnl!())
}
