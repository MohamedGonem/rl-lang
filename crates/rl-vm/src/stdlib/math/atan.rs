use crate::Vm;

pub fn std_atan(_: &mut Vm, x: f64) -> f64 {
    x.atan()
}
