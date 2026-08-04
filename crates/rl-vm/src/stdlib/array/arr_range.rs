use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_arr_range(_: &mut Vm, start: i64, end: i64, step: i64) -> VmValue {
    if step <= 0 {
        return verr!(vs!(format!(
            "arr_range: step must be positive, got {}",
            step
        )));
    }

    vok!(VmValue::Arr(Rc::new(
        (start..end)
            .step_by(step as usize)
            .map(VmValue::Int)
            .collect(),
    )))
}
