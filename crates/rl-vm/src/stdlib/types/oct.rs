use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn func(_: &mut Vm, value: VmValue) -> VmValue {
    let result = match value {
        VmValue::Int(v) => format!("{:o}", v),
        VmValue::Byte(v) => format!("{:o}", v),
        VmValue::Char(v) => format!("{:o}", v as u32),
        VmValue::Str(s) => s.bytes().map(|b| format!("{:o}", b)).collect::<String>(),

        other => {
            return verr!(vs!(format!(
                "cannot parse \"{}\" as octal",
                other.type_name()
            )));
        }
    };
    vok!(vs!(result))
}
