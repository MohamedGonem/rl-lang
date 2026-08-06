use crate::{
    stdlib::debug::common::assert_cmp,
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn func(eval: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    assert_cmp(eval, args, "assert_le", |a, b| a <= b)
}
