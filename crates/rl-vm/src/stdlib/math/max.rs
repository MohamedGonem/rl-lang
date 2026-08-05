use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_max(_: &mut Vm, a: VmValue, b: VmValue) -> VmValue {
    match (a, b) {
        (VmValue::Int(a), VmValue::Int(b)) => vok!(vi!(a.max(b))),
        (VmValue::Float(a), VmValue::Float(b)) => vok!(vf!(a.max(b))),
        (a, b) => verr!(vs!(format!(
            "max expects a number, got ({}, {})",
            a.type_name(),
            b.type_name()
        ))),
    }
}
