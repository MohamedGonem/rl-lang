use crate::Vm;

pub fn std_asin(_: &mut Vm, x: f64) -> f64 {
    x.asin()
}
