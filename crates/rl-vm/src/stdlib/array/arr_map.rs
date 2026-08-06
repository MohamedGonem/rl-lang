use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use std::rc::Rc;

pub fn std_arr_map(eval: &mut Vm, array: VmValue, function: VmValue) -> Result<VmValue, VmError> {
    let items = match array {
        VmValue::Arr(items) => items,
        other => {
            return Ok(verr!(vs!(format!(
                "arr_map: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };
    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return Ok(verr!(vs!(format!(
            "arr_map: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    let mut result = Vec::with_capacity(items.len());

    for item in items.iter() {
        let mapped_item =
            eval.call_value(&function, std::slice::from_ref(item), eval.current_span())?;
        result.push(mapped_item);
    }

    Ok(vok!(VmValue::Arr(Rc::new(result))))
}
