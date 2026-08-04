use crate::{Vm, stdlib::macros::vs, values::VmValue};

pub fn func(_: &mut Vm, v: VmValue) -> VmValue {
    vs!(v.type_name().to_string())
}
