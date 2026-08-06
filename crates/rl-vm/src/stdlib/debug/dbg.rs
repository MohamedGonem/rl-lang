use crate::{values::VmValue, vm_logic::Vm};

pub fn func(_: &mut Vm, value: VmValue) -> VmValue {
    let text = format!("[dbg] {} ({})\n", value, value.type_name());
    eprint!("{}", text);
    value
}
