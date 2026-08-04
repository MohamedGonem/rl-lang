use crate::Vm;

pub fn std_phi(_: &mut Vm) -> f64 {
    std::f64::consts::GOLDEN_RATIO
}
