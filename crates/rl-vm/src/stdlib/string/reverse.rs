use crate::Vm;

pub fn std_reverse(_: &mut Vm, string: String) -> String {
    string.chars().rev().collect()
}
