use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_bit_not(_: &mut Vm, v: VmValue) -> VmValue {
    match v {
        VmValue::Byte(x) => vok!(vby!(!x)),
        VmValue::Int(x) => vok!(vi!(!x)),
        _ => verr!(vs!("bit_not expects a byte or an int".to_string())),
    }
}
