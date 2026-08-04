use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use std::rc::Rc;

pub fn std_arr_sort_by(
    eval: &mut Vm,
    array: VmValue,
    function: VmValue,
) -> Result<VmValue, VmError> {
    let mut items = match array {
        VmValue::Arr(items) => (*items).clone(),
        other => {
            return Ok(verr!(vs!(format!(
                "arr_sort_by: accepts only arrays, found {}",
                other.type_name()
            ))));
        }
    };

    if !matches!(
        &function,
        VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. }
    ) {
        return Ok(verr!(vs!(format!(
            "arr_sort_by: expected function or lambda, found {}",
            function.type_name()
        ))));
    }

    for i in 1..items.len() {
        let mut j = i;
        while j > 0 {
            let result = eval.call_value(
                function.clone(),
                vec![items[j - 1].clone(), items[j].clone()],
                eval.current_span(),
            )?;

            match result {
                VmValue::Int(n) if n > 0 => {
                    items.swap(j - 1, j);
                    j -= 1;
                }
                VmValue::Int(_) => break,
                other => {
                    return Ok(verr!(vs!(format!(
                        "arr_sort_by: comparator must return int (-1, 0, 1), found {}",
                        other.type_name()
                    ))));
                }
            }
        }
    }

    Ok(vok!(VmValue::Arr(Rc::new(items))))
}
