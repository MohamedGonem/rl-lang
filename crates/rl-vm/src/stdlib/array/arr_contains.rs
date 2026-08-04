use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_contains(_: &mut Vm, array: VmValue, value: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => vok!(VmValue::Bool(items.contains(&value))),
        other => verr!(vs!(format!(
            "arr_contains: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
