use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn std_arr_reduce(
    eval: &mut Vm,
    array: VmValue,
    function: VmValue,
    initial: VmValue,
) -> Result<VmValue, VmError> {
    let items = match array {
        VmValue::Arr(items) => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_reduce: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return Ok(verr!(vs!(format!(
            "arr_reduce: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    let mut result = initial;

    for item in items.iter() {
        result = eval.call_value(
            &function,
            &[result, (*item).clone()],
            eval.current_span(),
        )?;
    }

    Ok(vok!(result))
}
