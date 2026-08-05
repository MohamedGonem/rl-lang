use crate::stdlib::macros::{try_fn, verr, vnl, vok, vs};
use crate::{Vm, values::VmValue};
use crossterm::{cursor::Show, execute};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_show_cursor", execute!(stdout(), Show));
    vok!(vnl!())
}
