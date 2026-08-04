use crate::{
    Vm,
    stdlib::macros::{try_fn, verr, vnl, vok, vs},
    values::VmValue,
};
use crossterm::{
    execute,
    terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
};
use std::io::stdout;

pub fn std_term_begin_sync(_: &mut Vm) -> VmValue {
    try_fn!(
        "term_begin_sync",
        execute!(stdout(), BeginSynchronizedUpdate)
    );
    vok!(vnl!())
}

pub fn std_term_end_sync(_: &mut Vm) -> VmValue {
    try_fn!("term_end_sync", execute!(stdout(), EndSynchronizedUpdate));
    vok!(vnl!())
}
