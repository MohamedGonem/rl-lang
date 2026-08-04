use crate::Vm;

pub fn std_e(_: &mut Vm) -> f64 {
    std::f64::consts::E
}
