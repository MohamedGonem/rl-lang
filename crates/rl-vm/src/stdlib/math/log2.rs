use crate::{
    Vm,
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
};

pub fn std_log2(_: &mut Vm, a: VmValue) -> VmValue {
    match a {
        VmValue::Int(i) => vok!(vf!((i as f64).log2())),
        VmValue::Float(f) => vok!(vf!(f.log2())),
        other => verr!(vs!(format!(
            "log2 expects a number, got {}",
            other.type_name()
        ))),
    }
}
