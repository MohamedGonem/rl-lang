use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_sum(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items.iter().any(|v| matches!(v, VmValue::Float(_))) {
                let sum = items
                    .iter()
                    .filter_map(|v| {
                        if let VmValue::Float(f) = v {
                            Some(*f)
                        } else {
                            None
                        }
                    })
                    .sum::<f64>();
                vok!(VmValue::Float(sum))
            } else {
                let sum = items
                    .iter()
                    .filter_map(|v| {
                        if let VmValue::Int(i) = v {
                            Some(*i)
                        } else {
                            None
                        }
                    })
                    .sum::<i64>();
                vok!(VmValue::Int(sum))
            }
        }
        other => verr!(vs!(format!(
            "arr_sum: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
