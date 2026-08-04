use crate::Vm;

pub fn func(eval: &mut Vm) -> i64 {
    eval.rng.generate_random_int_range(i64::MIN, i64::MAX)
}
