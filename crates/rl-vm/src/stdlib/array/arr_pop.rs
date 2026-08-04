use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_pop(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items.is_empty() {
                return verr!(vs!("arr_pop: called on empty array".to_string()));
            }
            let mut v = (*items).clone();
            v.pop();
            vok!(VmValue::Arr(Rc::new(v)))
        }
        other => verr!(vs!(format!(
            "arr_pop: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
