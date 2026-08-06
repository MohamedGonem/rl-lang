use crate::{
    Vm,
    stdlib::common::{vb, verr, vok, vs},
    values::VmValue,
};

pub fn std_is_bool(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Bool(_))
}

pub fn std_to_bool(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Bool(b) => b,
        VmValue::Int(i) => i != 0,
        VmValue::Byte(i) => i != 0,
        VmValue::Float(f) => f != 0.0,
        VmValue::Null => false,
        VmValue::Str(s) => !matches!(s.trim(), "false" | "0" | ""),

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as bool",
                other.type_name()
            )));
        }
    };

    vok!(vb!(result))
}
