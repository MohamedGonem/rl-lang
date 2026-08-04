use crate::{
    Vm,
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
};

pub fn std_log(_: &mut Vm, a: VmValue, base: VmValue) -> VmValue {
    match (a, base) {
        (VmValue::Int(i), VmValue::Int(base)) => vok!(vf!((i as f64).log(base as f64))),
        (VmValue::Float(f), VmValue::Float(base)) => vok!(vf!(f.log(base))),
        (VmValue::Float(f), VmValue::Int(base)) => vok!(vf!(f.log(base as f64))),
        (VmValue::Int(i), VmValue::Float(base)) => vok!(vf!((i as f64).log(base))),

        (a, base) => verr!(vs!(format!(
            "log expects a number, got ({}, {})",
            a.type_name(),
            base.type_name()
        ))),
    }
}
