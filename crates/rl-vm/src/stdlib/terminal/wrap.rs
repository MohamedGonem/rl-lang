use crate::stdlib::macros::{try_fn, verr, vnl, vok, vs};
use crate::{Vm, values::VmValue};

use crossterm::{
    execute,
    terminal::{DisableLineWrap, EnableLineWrap},
};
use std::io::stdout;

pub fn std_term_enable_wrap(_: &mut Vm) -> VmValue {
    try_fn!("term_enable_wrap", execute!(stdout(), EnableLineWrap));
    vok!(vnl!())
}

pub fn std_term_disable_wrap(_: &mut Vm) -> VmValue {
    try_fn!("term_disable_wrap", execute!(stdout(), DisableLineWrap));
    vok!(vnl!())
}
