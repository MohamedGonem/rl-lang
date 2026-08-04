use crate::Vm;

pub fn std_acos(_: &mut Vm, x: f64) -> f64 {
    x.acos()
}
