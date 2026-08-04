use crate::stdlib::macros::{try_fn, verr, vnl, vok, vs};
use crate::stdlib::terminal::common::extract_byte;
use crate::{Vm, values::VmValue};
use crossterm::{
    execute,
    style::{Color, SetBackgroundColor},
};
use std::io::stdout;

pub fn func(_: &mut Vm, r: VmValue, g: VmValue, b: VmValue) -> VmValue {
    let r = match extract_byte(r, "r") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let b = match extract_byte(b, "b") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };
    let g = match extract_byte(g, "r") {
        Ok(v) => v,
        Err(e) => return verr!(vs!(e)),
    };

    try_fn!(
        "term_set_bg",
        execute!(stdout(), SetBackgroundColor(Color::Rgb { r, g, b }))
    );
    vok!(vnl!())
}
