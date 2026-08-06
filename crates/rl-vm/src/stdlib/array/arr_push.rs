use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_push(_: &mut Vm, array: VmValue, value: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            let mut v = (*items).clone();
            v.push(value);
            vok!(VmValue::Arr(Rc::new(v)))
        }
        other => verr!(vs!(format!(
            "arr_push: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
