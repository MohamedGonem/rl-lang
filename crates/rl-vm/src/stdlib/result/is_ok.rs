use crate::Vm;
use crate::values::VmValue;

pub fn func(_: &mut Vm, value: VmValue) -> bool {
    value.is_ok()
}
