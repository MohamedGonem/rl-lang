use crate::{
    Vm,
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
};

pub fn std_sqrt(_: &mut Vm, a: VmValue) -> VmValue {
    match a {
        VmValue::Int(i) => vok!(vf!((i as f64).sqrt())),
        VmValue::Float(f) => vok!(vf!(f.sqrt())),
        other => verr!(vs!(format!(
            "sqrt expects a number, got {}",
            other.type_name()
        ))),
    }
}
