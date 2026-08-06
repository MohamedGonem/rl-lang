use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn std_arr_find_index(
    eval: &mut Vm,
    array: VmValue,
    function: VmValue,
) -> Result<VmValue, VmError> {
    let items = match array {
        VmValue::Arr(items) => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_find_index: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return Ok(verr!(vs!(format!(
            "arr_find_index: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    for (i, item) in items.iter().enumerate() {
        let mapped_item =
            eval.call_value(&function, std::slice::from_ref(item), eval.current_span())?;
        if let VmValue::Bool(true) = mapped_item {
            return Ok(vok!(VmValue::Int(i as i64)));
        }
    }

    Ok(vok!(VmValue::Int(-1_i64)))
}
