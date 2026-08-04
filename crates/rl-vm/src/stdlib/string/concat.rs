use crate::{
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use std::rc::Rc;

pub fn std_concat(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    Ok(VmValue::Str(Rc::from(
        args.iter().map(|a| a.to_string()).collect::<String>(),
    )))
}
