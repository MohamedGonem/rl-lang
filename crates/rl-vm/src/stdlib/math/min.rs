use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_min(_: &mut Vm, a: VmValue, b: VmValue) -> VmValue {
    match (a, b) {
        (VmValue::Int(a), VmValue::Int(b)) => vok!(vi!(a.min(b))),
        (VmValue::Float(a), VmValue::Float(b)) => vok!(vf!(a.min(b))),
        (a, b) => verr!(vs!(format!(
            "min expects a number, got ({}, {})",
            a.type_name(),
            b.type_name()
        ))),
    }
}
