use crate::Vm;

pub fn std_cos(_: &mut Vm, x: f64) -> f64 {
    x.cos()
}
