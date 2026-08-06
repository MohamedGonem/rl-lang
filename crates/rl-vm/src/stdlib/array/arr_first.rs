use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_first(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => match items.iter().next() {
            Some(v) => vok!((*v).clone()),
            None => verr!(vs!("arr_first: called on empty array".to_string())),
        },
        other => verr!(vs!(format!(
            "arr_first: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
