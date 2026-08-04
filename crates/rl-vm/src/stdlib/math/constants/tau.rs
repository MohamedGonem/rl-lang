use crate::Vm;

pub fn std_tau(_: &mut Vm) -> f64 {
    std::f64::consts::TAU
}
