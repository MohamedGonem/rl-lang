use crate::Vm;

pub fn std_factorial(_: &mut Vm, x: i64) -> i64 {
    (1..=x).product()
}
