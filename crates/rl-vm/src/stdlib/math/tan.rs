use crate::Vm;

pub fn std_tan(_: &mut Vm, x: f64) -> f64 {
    x.tan()
}
