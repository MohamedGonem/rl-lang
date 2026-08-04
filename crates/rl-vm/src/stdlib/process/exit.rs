use crate::{Vm, values::VmValue};

pub fn std_exit(_: &mut Vm, code: i64) -> VmValue {
    std::process::exit(code as i32);
}
