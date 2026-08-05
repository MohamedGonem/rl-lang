use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn std_arr_for_each(
    eval: &mut Vm,
    array: VmValue,
    function: VmValue,
) -> Result<VmValue, VmError> {
    let items = match array {
        VmValue::Arr(items) => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_for_each: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return Ok(verr!(vs!(format!(
            "arr_for_each: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    for item in items.iter() {
        eval.call_value(function.clone(), vec![(*item).clone()], eval.current_span())?;
    }

    Ok(vok!(VmValue::Null))
}
