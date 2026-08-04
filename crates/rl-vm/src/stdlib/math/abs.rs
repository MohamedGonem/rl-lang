use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

/// returns the absolute value of number
pub fn std_abs(_: &mut Vm, a: VmValue) -> VmValue {
    match a {
        VmValue::Int(i) => vok!(vi!(i.abs())),
        VmValue::Float(f) => vok!(vf!(f.abs())),
        other => verr!(vs!(format!(
            "abs() expects a number, got {}",
            other.type_name()
        ))),
    }
}
