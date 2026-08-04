use crate::Vm;

pub fn std_radians(_: &mut Vm, x: f64) -> f64 {
    x.to_radians()
}
