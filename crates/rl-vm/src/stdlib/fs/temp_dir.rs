use std::env::temp_dir;

use crate::{Vm, stdlib::macros::vs, values::VmValue};

pub fn std_temp_dir(_: &mut Vm) -> VmValue {
    vs!(temp_dir().to_string_lossy().to_string())
}
