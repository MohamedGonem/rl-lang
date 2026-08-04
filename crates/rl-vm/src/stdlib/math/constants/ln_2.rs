use crate::Vm;

pub fn std_ln_2(_: &mut Vm) -> f64 {
    std::f64::consts::LN_2
}
