use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_trailing_zeros(_: &mut Vm, v: VmValue) -> VmValue {
    match v {
        VmValue::Byte(x) => vok!(VmValue::Byte(u8::trailing_zeros(x) as u8)),
        VmValue::Int(x) => vok!(VmValue::Int(i64::trailing_zeros(x) as i64)),
        _ => verr!(vs!("trailing_zeros expects a byte or an int".to_string())),
    }
}
