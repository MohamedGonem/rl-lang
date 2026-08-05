use crate::Vm;

pub fn std_contains(_: &mut Vm, string: String, sub: String) -> bool {
    string.contains(&sub)
}
