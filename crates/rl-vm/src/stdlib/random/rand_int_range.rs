use crate::{
    Vm,
    stdlib::macros::{verr, vi, vok, vs},
    values::VmValue,
};

pub fn func(eval: &mut Vm, min: i64, max: i64) -> VmValue {
    if min >= max {
        return verr!(vs!(
            "min value shouldn't be bigger than or equal to maximum value".to_string()
        ));
    }

    vok!(vi!(eval.rng.generate_random_int_range(min, max)))
}
