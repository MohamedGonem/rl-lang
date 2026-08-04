use crate::{
    Vm,
    stdlib::macros::{vc, verr, vok, vs},
    values::VmValue,
};

pub fn std_is_char(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Char(_))
}

pub fn std_to_char(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Char(c) => c,
        VmValue::Int(i) => match char::from_u32(i as u32) {
            Some(c) => c,
            None => return verr!(vs!(format!("{} is not a valid unicode codepoint", i))),
        },
        VmValue::Byte(i) => match char::from_u32(i as u32) {
            Some(c) => c,
            None => return verr!(vs!(format!("{} is not a valid unicode codepoint", i))),
        },
        VmValue::Str(s) => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => c,
                _ => return verr!(vs!("string must be exactly one character".to_string())),
            }
        }

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as character",
                other.type_name()
            )));
        }
    };

    vok!(vc!(result))
}
