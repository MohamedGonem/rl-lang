use crate::{Vm, values::VmValue};
use std::rc::Rc;

pub fn std_arr_fill(_: &mut Vm, value: VmValue, count: i64) -> VmValue {
    VmValue::Arr(Rc::new(vec![value; count as usize]))
}
