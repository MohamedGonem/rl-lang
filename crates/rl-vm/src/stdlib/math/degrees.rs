use crate::Vm;

pub fn std_degrees(_: &mut Vm, x: f64) -> f64 {
    x.to_degrees()
}
