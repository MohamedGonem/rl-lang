use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_mod(_: &mut Vm, a: VmValue, b: VmValue) -> VmValue {
    match (a, b) {
        (VmValue::Int(a), VmValue::Int(b)) => vok!(vi!(a % b)),
        (VmValue::Float(a), VmValue::Float(b)) => vok!(vf!(a % b)),
        (VmValue::Int(a), VmValue::Float(b)) => vok!(vf!(a as f64 % b)),
        (VmValue::Float(a), VmValue::Int(b)) => vok!(vf!(a % b as f64)),
        (a, b) => verr!(vs!(format!(
            "mod expects a number, got ({}, {})",
            a.type_name(),
            b.type_name()
        ))),
    }
}
