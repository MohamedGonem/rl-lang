use crate::{
    Vm,
    stdlib::macros::{vby, verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_count_bits(_: &mut Vm, v: VmValue) -> VmValue {
    match v {
        VmValue::Byte(x) => vok!(vby!(x.count_ones() as u8)),
        VmValue::Int(x) => vok!(vi!(x.count_ones() as i64)),
        _ => verr!(vs!("count_bits expects a byte or an int".to_string())),
    }
}
