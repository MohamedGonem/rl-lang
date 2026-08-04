use crate::Vm;

pub fn func(eval: &mut Vm, weight: f64) -> bool {
    eval.rng.generate_random_bool(weight)
}
