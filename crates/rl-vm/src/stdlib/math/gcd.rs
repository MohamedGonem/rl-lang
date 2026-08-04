use crate::Vm;

pub fn std_gcd(_: &mut Vm, x: i64, y: i64) -> i64 {
    let mut a = x as u64;
    let mut b = y as u64;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a as i64
}
