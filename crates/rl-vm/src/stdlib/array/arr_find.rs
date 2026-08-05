use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn std_arr_find(eval: &mut Vm, array: VmValue, function: VmValue) -> Result<VmValue, VmError> {
    let items = match array {
        VmValue::Arr(items) => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_find: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return Ok(verr!(vs!(format!(
            "arr_find: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    for item in items.iter() {
        let mapped_item =
            eval.call_value(&function, std::slice::from_ref(item), eval.current_span())?;
        if let VmValue::Bool(true) = mapped_item {
            return Ok(vok!((*item).clone()));
        }
    }

    Ok(vok!(VmValue::Null))
}
