use crate::Vm;

pub fn std_lerp(_: &mut Vm, x: f64, y: f64, t: f64) -> f64 {
    x + (y - x) * t
}
