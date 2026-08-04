use crate::Vm;

pub fn std_hypot(_: &mut Vm, x: f64, y: f64) -> f64 {
    x.hypot(y)
}
