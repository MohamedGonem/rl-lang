use crate::stdlib::macros::{verr, vnl, vok, vs};
use crate::{Vm, values::VmValue};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn func(_: &mut Vm) -> VmValue {
    match execute!(stdout(), Clear(ClearType::All)) {
        Err(e) => verr!(vs!(format!("term_clear(): {}", e))),
        Ok(_) => vok!(vnl!()),
    }
}
