use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_min(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items.iter().any(|v| matches!(v, VmValue::Float(_))) {
                match items
                    .iter()
                    .filter_map(|v| {
                        if let VmValue::Float(f) = v {
                            Some(*f)
                        } else {
                            None
                        }
                    })
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                {
                    Some(min) => vok!(VmValue::Float(min)),
                    None => verr!(vs!("arr_min: called on empty array".to_string())),
                }
            } else {
                match items
                    .iter()
                    .filter_map(|v| {
                        if let VmValue::Int(i) = v {
                            Some(*i)
                        } else {
                            None
                        }
                    })
                    .min()
                {
                    Some(min) => vok!(VmValue::Int(min)),
                    None => verr!(vs!("arr_min: called on empty array".to_string())),
                }
            }
        }
        other => verr!(vs!(format!(
            "arr_min: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
