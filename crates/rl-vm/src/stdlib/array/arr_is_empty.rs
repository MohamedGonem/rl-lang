use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_is_empty(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => vok!(VmValue::Bool(items.is_empty())),
        other => verr!(vs!(format!(
            "arr_is_empty: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
