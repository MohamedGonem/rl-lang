use crate::Vm;

pub fn std_trim(_: &mut Vm, string: String) -> String {
    string.trim().to_string()
}
