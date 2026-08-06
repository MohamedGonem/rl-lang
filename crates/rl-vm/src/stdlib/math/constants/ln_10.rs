use crate::Vm;

pub fn std_ln_10(_: &mut Vm) -> f64 {
    std::f64::consts::LN_10
}
