use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_flatten(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => vok!(VmValue::Arr(Rc::new(
            items
                .iter()
                .flat_map(|v| {
                    if let VmValue::Arr(inner) = v {
                        (**inner).clone()
                    } else {
                        vec![(*v).clone()]
                    }
                })
                .collect(),
        ))),
        other => verr!(vs!(format!(
            "arr_flatten: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
