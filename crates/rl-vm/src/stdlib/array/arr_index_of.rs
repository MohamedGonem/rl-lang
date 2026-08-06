use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_index_of(_: &mut Vm, array: VmValue, value: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => match items.iter().position(|item| *item == value) {
            Some(pos) => vok!(VmValue::Int(pos as i64)),
            None => vok!(VmValue::Int(-1)),
        },
        other => verr!(vs!(format!(
            "arr_index_of: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
