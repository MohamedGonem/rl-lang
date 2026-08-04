use crate::Vm;

pub fn std_atan2(_: &mut Vm, x: f64, y: f64) -> f64 {
    y.atan2(x)
}
