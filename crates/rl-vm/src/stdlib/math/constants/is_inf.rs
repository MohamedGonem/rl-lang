use crate::Vm;

pub fn std_is_inf(_: &mut Vm, x: f64) -> bool {
    x.is_infinite()
}
