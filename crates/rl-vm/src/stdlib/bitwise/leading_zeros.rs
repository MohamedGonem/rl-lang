use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_leading_zeros(_: &mut Vm, v: VmValue) -> VmValue {
    match v {
        VmValue::Byte(x) => vok!(vby!(u8::leading_zeros(x) as u8)),
        VmValue::Int(x) => vok!(vi!(i64::leading_zeros(x) as i64)),
        _ => verr!(vs!("leading_zeros expects a byte or an int".to_string())),
    }
}
