use crate::stdlib::common::{extract_number, vb, verr, vok, vs};
use crate::{Vm, values::VmValue};
use crossterm::event::poll;
use std::time::Duration;

pub fn func(_: &mut Vm, arg: VmValue) -> VmValue {
    let ms = match extract_number(arg, "ms") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    let ready = match poll(Duration::from_millis(ms)) {
        Ok(v) => v,
        Err(e) => return verr!(vs!(format!("term_poll(): {}", e))),
    };
    vok!(vb!(ready))
}
