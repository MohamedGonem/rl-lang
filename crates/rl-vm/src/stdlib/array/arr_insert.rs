use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_insert(_: &mut Vm, array: VmValue, value: VmValue, index: i64) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if index < 0 || index as usize > items.len() {
                return verr!(vs!(format!("arr_insert: index out of bounds: {}", index)));
            }
            let mut v = (*items).clone();
            v.insert(index as usize, value);
            vok!(VmValue::Arr(Rc::new(v)))
        }
        other => verr!(vs!(format!(
            "arr_insert: accepts only arrays and values, found {}",
            other.type_name()
        ))),
    }
}
