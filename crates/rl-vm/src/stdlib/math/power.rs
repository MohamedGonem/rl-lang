use crate::{
    Vm,
    stdlib::macros::{verr, vf, vi, vok, vs},
    values::VmValue,
};

pub fn std_pow(_: &mut Vm, base: VmValue, exponent: VmValue) -> VmValue {
    match (base, exponent) {
        (VmValue::Int(a), VmValue::Int(b)) => {
            let b = b as u32;
            vok!(vi!(a.pow(b)))
        }
        (VmValue::Int(a), VmValue::Float(b)) => vok!(vf!((a as f64).powf(b))),
        (VmValue::Float(a), VmValue::Float(b)) => vok!(vf!(a.powf(b))),
        (VmValue::Float(a), VmValue::Int(b)) => vok!(vf!(a.powi(b as i32))),
        _ => verr!(vs!("pow expects numeric arguments".to_string())),
    }
}
