use crate::{Vm, stdlib::macros::vi, values::VmValue};

pub fn std_pid(_: &mut Vm) -> VmValue {
    vi!(std::process::id() as i64)
}
