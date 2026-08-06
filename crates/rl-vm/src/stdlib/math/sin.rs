use crate::Vm;

pub fn std_sin(_: &mut Vm, x: f64) -> f64 {
    x.sin()
}
