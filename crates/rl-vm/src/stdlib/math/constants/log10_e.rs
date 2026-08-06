use crate::Vm;

pub fn std_log10_e(_: &mut Vm) -> f64 {
    std::f64::consts::LOG10_E
}
