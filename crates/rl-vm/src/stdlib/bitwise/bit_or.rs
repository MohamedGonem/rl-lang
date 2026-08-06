use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_bit_or(_: &mut Vm, a: VmValue, b: VmValue) -> VmValue {
    match (a, b) {
        (VmValue::Byte(x), VmValue::Byte(y)) => vok!(vby!(x | y)),
        (VmValue::Int(x), VmValue::Int(y)) => vok!(vi!(x | y)),
        (VmValue::Byte(x), VmValue::Int(y)) => vok!(vi!(x as i64 | y)),
        (VmValue::Int(x), VmValue::Byte(y)) => vok!(vi!(x | y as i64)),
        _ => verr!(vs!("bit_or expects byte or integer arguments".to_string())),
    }
}
