use crate::Vm;

pub fn std_is_nan(_: &mut Vm, x: f64) -> bool {
    x.is_nan()
}
