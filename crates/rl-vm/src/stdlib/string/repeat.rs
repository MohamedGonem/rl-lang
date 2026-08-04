use crate::Vm;

pub fn std_repeat(_: &mut Vm, string: String, count: i64) -> String {
    string.repeat(count as usize)
}
