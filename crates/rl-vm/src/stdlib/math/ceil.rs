use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_ceil(_: &mut Vm, a: VmValue) -> VmValue {
    match a {
        VmValue::Int(i) => vok!(vi!(i)),
        VmValue::Float(f) => vok!(vf!(f.ceil())),
        other => verr!(vs!(format!(
            "ceil() expects a number, got {}",
            other.type_name()
        ))),
    }
}
