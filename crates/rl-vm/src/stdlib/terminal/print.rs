use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{execute, style::Print};
use std::io::stdout;

pub fn func(_: &mut Vm, arg: VmValue) -> VmValue {
    let text = arg.to_string();

    try_fn!("term_print", execute!(stdout(), Print(text)));
    vok!(vnl!())
}
