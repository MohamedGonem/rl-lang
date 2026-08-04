use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_remove(_: &mut Vm, array: VmValue, index: i64) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if index as usize >= items.len() {
                return verr!(vs!(format!("arr_remove: index out of bounds: {}", index)));
            }
            let mut v = (*items).clone();
            v.remove(index as usize);
            vok!(VmValue::Arr(Rc::new(v)))
        }
        other => verr!(vs!(format!(
            "arr_remove: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
