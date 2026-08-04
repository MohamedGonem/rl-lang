use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_arr_product(_: &mut Vm, array: VmValue) -> VmValue {
    match array {
        VmValue::Arr(items) => {
            if items.iter().any(|v| matches!(v, VmValue::Float(_))) {
                let product = items
                    .iter()
                    .filter_map(|v| {
                        if let VmValue::Float(f) = v {
                            Some(*f)
                        } else {
                            None
                        }
                    })
                    .product::<f64>();
                vok!(VmValue::Float(product))
            } else {
                let product = items
                    .iter()
                    .filter_map(|v| {
                        if let VmValue::Int(i) = v {
                            Some(*i)
                        } else {
                            None
                        }
                    })
                    .product::<i64>();
                vok!(VmValue::Int(product))
            }
        }
        other => verr!(vs!(format!(
            "arr_product: accepts only arrays, found {}",
            other.type_name()
        ))),
    }
}
