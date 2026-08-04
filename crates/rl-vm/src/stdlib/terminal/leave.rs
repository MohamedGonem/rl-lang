use crate::stdlib::macros::{try_fn, verr, vnl, vok, vs};
use crate::{Vm, values::VmValue};
use crossterm::cursor::Show;
use crossterm::{
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::io::{Write, stderr, stdout};

pub fn func(_: &mut Vm) -> VmValue {
    let _ = stdout().flush();
    let _ = stderr().flush();
    try_fn!("term_leave", disable_raw_mode());
    try_fn!("term_leave", execute!(stdout(), Show, LeaveAlternateScreen));
    let _ = stdout().flush();

    vok!(vnl!())
}
