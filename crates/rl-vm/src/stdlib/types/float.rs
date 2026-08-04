use crate::{
    Vm,
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
};

pub fn std_is_float(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Float(_))
}

pub fn std_to_float(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Float(f) => f,
        VmValue::Int(i) => i as f64,
        VmValue::Byte(i) => i as f64,
        VmValue::Bool(b) => {
            if b {
                1.0
            } else {
                0.0
            }
        }
        VmValue::Str(s) => match s.trim().parse::<f64>() {
            Ok(f) => f,
            Err(_) => return verr!(vs!(format!("cannot parse \"{}\" as float", s))),
        },

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as float",
                other.type_name()
            )));
        }
    };

    vok!(vf!(result))
}
