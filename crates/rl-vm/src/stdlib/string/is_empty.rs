use crate::Vm;

pub fn std_is_empty(_: &mut Vm, string: String) -> bool {
    string.is_empty()
}
