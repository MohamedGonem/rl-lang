use crate::Vm;

pub fn std_to_lower(_: &mut Vm, string: String) -> String {
    string.to_lowercase()
}
