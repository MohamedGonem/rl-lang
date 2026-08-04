use crate::Vm;

pub fn std_replace(_: &mut Vm, string: String, from: String, to: String) -> String {
    string.replace(&from, &to)
}
