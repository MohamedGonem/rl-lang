use crate::{values::VmValue, vm_logic::{Vm, VmError}};

pub fn std_eprint(vm: &mut Vm, string: String) -> Result<VmValue, VmError> {
    Err(vm.err(string))
}
