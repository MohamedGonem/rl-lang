use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_is_error(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Error(_))
}

pub fn std_error_unwrap(_: &mut Vm, value: VmValue) -> VmValue {
    match value {
        VmValue::Error(inner) => vok!(*inner),
        other => verr!(vs!(format!(
            "error_unwrap: expected error, got {}",
            other.type_name()
        ))),
    }
}
