use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_slice(_: &mut Vm, array: VmValue, start: i64, end: i64) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            let start = start as usize;
            let end = end as usize;
            if start > items.len() || end > items.len() {
                return verr!(vs!(format!(
                    "arr_slice: index out of bounds: {}..{} (len {})",
                    start,
                    end,
                    items.len()
                )));
            }
            if start > end {
                return verr!(vs!(format!(
                    "arr_slice: start {} is greater than end {}",
                    start, end
                )));
            }
            vok!(VmValue::Arr(Rc::new(items[start..end].to_vec())))
        }
        other => verr!(vs!(format!(
            "arr_slice: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
