use crate::Vm;

pub fn std_starts_with(_: &mut Vm, string: String, sub: String) -> bool {
    string.starts_with(&sub)
}
