use crate::Vm;

pub fn std_to_upper(_: &mut Vm, string: String) -> String {
    string.to_uppercase()
}
