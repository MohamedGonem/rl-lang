use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_is_string(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Str(_))
}

pub fn std_to_string(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Int(v) => format!("{}", v),
        VmValue::Byte(v) => format!("{}", v),
        VmValue::Float(v) => format!("{}", v),
        VmValue::Bool(v) => format!("{}", v),
        VmValue::Char(v) => v.to_string(),
        VmValue::Str(s) => s.to_string(),

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as string",
                other.type_name()
            )));
        }
    };
    vok!(vs!(result))
}
