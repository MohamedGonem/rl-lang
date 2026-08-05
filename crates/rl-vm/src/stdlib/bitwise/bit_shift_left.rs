use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_bit_shift_left(_: &mut Vm, a: VmValue, shift: VmValue) -> VmValue {
    match (a, shift) {
        (VmValue::Byte(x), VmValue::Byte(s)) => vok!(vby!(x << (s as u32))),
        (VmValue::Byte(x), VmValue::Int(s)) => vok!(vby!(x << (s as u32))),
        (VmValue::Int(x), VmValue::Byte(s)) => vok!(vi!(x << (s as u32))),
        (VmValue::Int(x), VmValue::Int(s)) => vok!(vi!(x << (s as u32))),
        _ => verr!(vs!(
            "bit_shift_left expects ((byte|int), (int|byte))".to_string()
        )),
    }
}
