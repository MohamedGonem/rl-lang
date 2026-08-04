use crate::{values::VmValue, vm_logic::Vm};
use std::rc::Rc;

pub fn std_args(eval: &mut Vm) -> VmValue {
    let args: Vec<VmValue> = std::env::args()
        .skip(eval.user_args_offset)
        .map(|s| VmValue::Str(Rc::from(s)))
        .collect();
    VmValue::Arr(Rc::new(args))
}
