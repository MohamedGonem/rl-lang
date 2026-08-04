use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_floor(_: &mut Vm, a: VmValue) -> VmValue {
    match a {
        VmValue::Int(i) => vok!(vi!(i)),
        VmValue::Float(f) => vok!(vf!(f.floor())),
        other => verr!(vs!(format!(
            "floor expects a number, got {}",
            other.type_name()
        ))),
    }
}
