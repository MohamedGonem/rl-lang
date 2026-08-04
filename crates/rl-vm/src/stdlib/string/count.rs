use crate::Vm;

pub fn std_count(_: &mut Vm, string: String, to_count: String) -> i64 {
    string.matches(&to_count).count() as i64
}
