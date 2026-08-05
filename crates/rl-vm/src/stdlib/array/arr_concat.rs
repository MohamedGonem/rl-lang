use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_concat(_: &mut Vm, array1: VmValue, array2: VmValue) -> VmValue {
    match (array1, array2) {
        (VmValue::Arr(i1), VmValue::Arr(i2)) => {
            let mut v = (*i1).clone();
            v.extend(i2.iter().cloned());
            vok!(VmValue::Arr(Rc::new(v)))
        }
        _ => verr!(vs!("arr_concat: accepts only arrays".to_string())),
    }
}
