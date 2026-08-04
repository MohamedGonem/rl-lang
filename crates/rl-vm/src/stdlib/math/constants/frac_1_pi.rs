use crate::Vm;

pub fn std_frac_1_pi(_: &mut Vm) -> f64 {
    std::f64::consts::FRAC_1_PI
}
