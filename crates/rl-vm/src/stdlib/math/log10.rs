use crate::{
    Vm,
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
};

pub fn std_log10(_: &mut Vm, a: VmValue) -> VmValue {
    match a {
        VmValue::Int(i) => vok!(vf!((i as f64).log10())),
        VmValue::Float(f) => vok!(vf!(f.log10())),
        other => verr!(vs!(format!(
            "log10 expects a number, got {}",
            other.type_name()
        ))),
    }
}
