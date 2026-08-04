use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_clamp(_: &mut Vm, value: VmValue, min: VmValue, max: VmValue) -> VmValue {
    match (value, min, max) {
        (VmValue::Int(value), VmValue::Int(low), VmValue::Int(high)) => {
            vok!(vi!(value.clamp(low, high)))
        }
        (VmValue::Float(value), VmValue::Float(low), VmValue::Float(high)) => {
            vok!(vf!(value.clamp(low, high)))
        }
        (value, min, max) => verr!(vs!(format!(
            "clamp expects a number, got ({}, {}, {})",
            value.type_name(),
            min.type_name(),
            max.type_name()
        ))),
    }
}
