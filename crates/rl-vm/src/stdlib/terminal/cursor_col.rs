use crate::stdlib::macros::{verr, vnl, vok, vs};
use crate::stdlib::terminal::common::extract_u16;
use crate::{Vm, values::VmValue};
use crossterm::{cursor::MoveToColumn, execute};
use std::io::stdout;

pub fn func(_: &mut Vm, args: VmValue) -> VmValue {
    let col = match extract_u16(args, "col") {
        Ok(a) => a,
        Err(e) => return verr!(vs!(e)),
    };
    match execute!(stdout(), MoveToColumn(col)) {
        Err(e) => verr!(vs!(format!("term_move_to_col(): {}", e))),
        Ok(_) => vok!(vnl!()),
    }
}
