use crate::{Vm, stdlib::macros::vs, values::VmValue};

pub fn func(_: &mut Vm) -> VmValue {
    vs!(env!("CARGO_PKG_VERSION").to_string())
}
