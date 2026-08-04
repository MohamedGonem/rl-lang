use crate::{values::VmValue, vm_logic::Vm};
use std::rc::Rc;

pub fn std_split(_: &mut Vm, string: String, delim: String) -> VmValue {
    VmValue::Arr(Rc::new(
        string
            .split(&delim)
            .map(|s| VmValue::Str(Rc::from(s)))
            .collect(),
    ))
}
