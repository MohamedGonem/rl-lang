use crate::Vm;

pub fn std_trim_end(_: &mut Vm, string: String) -> String {
    string.trim_end().to_string()
}
