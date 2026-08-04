use crate::{Vm, values::VmValue};

pub fn func(_: &mut Vm, value: VmValue) -> bool {
    matches!(value, VmValue::Null)
}
