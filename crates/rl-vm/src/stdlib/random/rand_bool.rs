use crate::Vm;

pub fn func(eval: &mut Vm) -> bool {
    let rand_float = eval.rng.generate_random_float();
    eval.rng.generate_random_bool(rand_float)
}
