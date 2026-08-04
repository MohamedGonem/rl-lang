use crate::stdlib::common::{try_fn, verr, vi, vnl, vok, vs};
use crate::stdlib::terminal::common::extract_u16;
use crate::{Vm, values::VmValue};
use crossterm::{
    execute,
    terminal::{SetSize, size},
};
use std::io::stdout;
use std::rc::Rc;

pub fn std_term_get_size(_: &mut Vm) -> VmValue {
    let (cols, rows) = match size() {
        Ok((cols, rows)) => (cols, rows),
        Err(e) => return verr!(vs!(format!("term_get_size(): {}", e))),
    };

    vok!(VmValue::Arr(Rc::new(vec![
        vi!(cols as i64),
        vi!(rows as i64)
    ],)))
}

pub fn std_term_set_size(_: &mut Vm, cols: VmValue, rows: VmValue) -> VmValue {
    let cols = match extract_u16(cols, "cols") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let rows = match extract_u16(rows, "rows") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!("term_set_size", execute!(stdout(), SetSize(cols, rows)));
    vok!(vnl!())
}
