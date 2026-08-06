use crate::{Vm, values::VmValue};
use std::rc::Rc;

pub fn std_chars(_: &mut Vm, string: String) -> VmValue {
    let chars = string.chars().map(VmValue::Char).collect();
    VmValue::Arr(Rc::new(chars))
}
