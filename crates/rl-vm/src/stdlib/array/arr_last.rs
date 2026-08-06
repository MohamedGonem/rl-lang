use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_last(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => match items.last() {
            Some(v) => vok!((*v).clone()),
            None => verr!(vs!("arr_last: called on empty array".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_last: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
