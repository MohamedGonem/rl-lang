use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn std_result_map(eval: &mut Vm, a: VmValue, b: VmValue) -> Result<VmValue, VmError> {
    match a {
        VmValue::Ok(inner) => {
            let mapped = match eval.call_value(&b, &[*inner], eval.current_span()) {
                Ok(mapped) => mapped,
                Err(e) => return Ok(verr!(vs!(e.message().to_string()))),
            };
            Ok(vok!(mapped))
        }
        // pass error as is
        VmValue::Err(_) => Ok(a),
        other => Ok(verr!(vs!(format!(
            "result_map: expected result, got {}",
            other.type_name()
        )))),
    }
}

pub fn std_result_map_err(eval: &mut Vm, a: VmValue, b: VmValue) -> Result<VmValue, VmError> {
    match a {
        VmValue::Err(inner) => {
            let mapped = match eval.call_value(&b, &[*inner], eval.current_span()) {
                Ok(mapped) => mapped,
                Err(e) => return Ok(verr!(vs!(e.message().to_string()))),
            };
            Ok(verr!(mapped))
        }
        // pass ok as is
        VmValue::Ok(_) => Ok(a),
        other => Ok(verr!(vs!(format!(
            "result_map_err: expected result, got {}",
            other.type_name()
        )))),
    }
}
