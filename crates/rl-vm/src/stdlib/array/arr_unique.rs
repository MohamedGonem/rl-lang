use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_unique(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            let mut seen = Vec::new();
            for item in items.iter() {
                if !seen.contains(item) {
                    seen.push((*item).clone());
                }
            }
            vok!(VmValue::Arr(Rc::new(seen)))
        }
        other => verr!(vs!(format!(
            "arr_unique: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
