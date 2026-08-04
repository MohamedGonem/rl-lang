use crate::Vm;

pub fn std_pi(_: &mut Vm) -> f64 {
    std::f64::consts::PI
}
