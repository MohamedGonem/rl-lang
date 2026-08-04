use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_bit_xor(_: &mut Vm, a: VmValue, b: VmValue) -> VmValue {
    match (a, b) {
        (VmValue::Byte(x), VmValue::Byte(y)) => vok!(vby!(x ^ y)),
        (VmValue::Int(x), VmValue::Int(y)) => vok!(vi!(x ^ y)),
        _ => verr!(vs!(
            "bit_xor expects (byte, byte) or (int, int) arguments".to_string()
        )),
    }
}
