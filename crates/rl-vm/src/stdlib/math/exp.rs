use crate::Vm;

pub fn std_exp(_: &mut Vm, x: f64) -> f64 {
    x.exp()
}
