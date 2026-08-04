use crate::{
    Vm,
    stdlib::macros::{verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_is_int(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Int(_))
}

pub fn std_to_int(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Int(v) => v,
        VmValue::Byte(v) => v as i64,
        VmValue::Float(v) => v as i64,
        VmValue::Bool(v) => {
            if v {
                1
            } else {
                0
            }
        }
        VmValue::Char(v) => v as i64,
        VmValue::Str(s) => {
            let s = s.trim();
            if s.starts_with("0x") || s.starts_with("0X") {
                match i64::from_str_radix(&s[2..], 16) {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as int", s))),
                }
            } else {
                match s.parse::<i64>() {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as int", s))),
                }
            }
        }

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as int",
                other.type_name()
            )));
        }
    };
    vok!(vi!(result))
}
