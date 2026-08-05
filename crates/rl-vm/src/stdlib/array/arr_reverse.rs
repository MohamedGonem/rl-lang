use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_reverse(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            let mut v = (*items).clone();
            v.reverse();
            vok!(VmValue::Arr(Rc::new(v)))
        }
        other => verr!(vs!(format!(
            "arr_reverse: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
