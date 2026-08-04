use crate::{Vm, values::VmValue};
use std::rc::Rc;

pub fn std_bytes(_: &mut Vm, string: String) -> VmValue {
    let bytes = string.bytes().map(VmValue::Byte).collect();
    VmValue::Arr(Rc::new(bytes))
}
