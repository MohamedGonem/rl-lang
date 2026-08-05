use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_count(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => vok!(VmValue::Int(items.len() as i64)),
        other => verr!(vs!(format!(
            "arr_count: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
