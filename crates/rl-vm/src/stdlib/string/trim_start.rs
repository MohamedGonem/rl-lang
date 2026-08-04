use crate::Vm;

pub fn std_trim_start(_: &mut Vm, string: String) -> String {
    string.trim_start().to_string()
}
