use crate::Vm;

pub fn func(eval: &mut Vm) -> f64 {
    eval.rng.generate_random_float()
}
