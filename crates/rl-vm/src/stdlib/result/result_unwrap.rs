use crate::{
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn std_unwrap(eval: &mut Vm, v: VmValue) -> Result<VmValue, VmError> {
    match v {
        VmValue::Ok(inner) => Ok(*inner),
        VmValue::Err(v) => Err(eval.err(format!("result_unwrap: called on Err({})", v))),
        other => Err(eval.err(format!(
            "result_unwrap: expected result, got {}",
            other.type_name()
        ))),
    }
}

pub fn std_unwrap_err(eval: &mut Vm, v: VmValue) -> Result<VmValue, VmError> {
    match v {
        VmValue::Err(inner) => Ok(*inner),
        VmValue::Ok(v) => Err(eval.err(format!("result_unwrap_err: called on ok({})", v))),
        other => Err(eval.err(format!(
            "result_unwrap_err: expected result, got {}",
            other.type_name()
        ))),
    }
}

pub fn std_unwrap_or(eval: &mut Vm, v: VmValue, b: VmValue) -> Result<VmValue, VmError> {
    match v {
        VmValue::Ok(inner) => Ok(*inner),
        VmValue::Err(_) => Ok(b),
        other => Err(eval.err(format!(
            "result_unwrap_or: expected result, got {}",
            other.type_name()
        ))),
    }
}
