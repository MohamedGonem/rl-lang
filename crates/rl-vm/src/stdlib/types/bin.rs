use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn func(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Byte(v) => format!("{:b}", v),
        VmValue::Int(v) => format!("{:b}", v),
        VmValue::Bool(v) => {
            if v {
                "1".to_string()
            } else {
                "0".to_string()
            }
        }
        VmValue::Char(v) => format!("{:b}", v as u32),
        VmValue::Str(s) => s.bytes().map(|b| format!("{:b}", b)).collect::<String>(),

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as binary",
                other.type_name()
            )));
        }
    };

    vok!(vs!(result))
}
