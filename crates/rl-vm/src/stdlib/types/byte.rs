use crate::{
    Vm,
    stdlib::common::{vby, verr, vok, vs},
    values::VmValue,
};

pub fn std_is_byte(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Byte(_))
}

pub fn std_to_byte(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Int(v) => v as u8,
        VmValue::Byte(v) => v,
        VmValue::Float(v) => v as u8,
        VmValue::Bool(v) => {
            if v {
                1u8
            } else {
                0u8
            }
        }
        VmValue::Char(v) => v as u8,
        VmValue::Str(s) => {
            let s = s.trim();
            if s.starts_with("0x") || s.starts_with("0X") {
                match u8::from_str_radix(&s[2..], 16) {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as byte", s))),
                }
            } else {
                match s.parse::<u8>() {
                    Ok(i) => i,
                    Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as byte", s))),
                }
            }
        }

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as byte",
                other.type_name()
            )));
        }
    };
    vok!(vby!(result))
}
