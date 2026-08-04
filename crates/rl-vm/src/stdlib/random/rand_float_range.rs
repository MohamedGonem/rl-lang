use crate::{
    Vm,
    stdlib::macros::{verr, vf, vok, vs},
    values::VmValue,
};

pub fn func(eval: &mut Vm, min: f64, max: f64) -> VmValue {
    if min >= max {
        return verr!(vs!(
            "min value shouldn't be bigger than or equal to maximum value".to_string()
        ));
    }

    vok!(vf!(eval.rng.generate_random_float_range(min, max)))
}
