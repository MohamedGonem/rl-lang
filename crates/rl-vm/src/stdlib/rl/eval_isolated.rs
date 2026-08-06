use crate::{
    stdlib::common::{extract_string, verr, vok, vs},
    stdlib::rl::common::compile_and_run,
    values::VmValue,
    vm_logic::Vm,
};

pub fn func(_: &mut Vm, value: VmValue) -> VmValue {
    let code = match extract_string(value, "eval_isolated") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(e)),
    };

    match compile_and_run(code, "<eval_isolated>") {
        Ok(v) => vok!(v),
        Err(e) => verr!(vs!(e)),
    }
}
