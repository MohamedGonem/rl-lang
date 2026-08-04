use crate::Vm;

pub fn std_ends_with(_: &mut Vm, string: String, sub: String) -> bool {
    string.ends_with(&sub)
}
