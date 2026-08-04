use crate::stdlib::macros::{try_fn, verr, vnl, vok, vs};
use crate::{Vm, values::VmValue};
use std::io::{Write, stdout};

pub fn func(_: &mut Vm) -> VmValue {
    try_fn!("term_flush", stdout().flush());

    vok!(vnl!())
}
