use crate::{
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use std::rc::Rc;

/// Zips two arrays into an array of tuples.
///
/// `arr_zip([1, 2, 3], ["a", "b", "c"])` → `[(1, "a"), (2, "b"), (3, "c")]`
///
/// Stops at the shorter array (same behaviour as Rust's `zip`).
pub fn std_arr_zip(eval: &mut Vm, array1: VmValue, array2: VmValue) -> Result<VmValue, VmError> {
    let (a, b) = match (&array1, &array2) {
        (VmValue::Arr(a), VmValue::Arr(b)) => (a, b),
        _ => {
            return Err(eval.err(format!(
                "arr_zip: expected two arrays, got {} and {}",
                array1.type_name(),
                array2.type_name()
            )));
        }
    };

    let items = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| VmValue::Tuple(Rc::new(vec![x.clone(), y.clone()])))
        .collect();

    Ok(VmValue::Arr(Rc::new(items)))
}
