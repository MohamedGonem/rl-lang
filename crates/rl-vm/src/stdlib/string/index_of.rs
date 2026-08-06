use crate::Vm;

pub fn std_index_of(_: &mut Vm, string: String, sub: String) -> i64 {
    match string.find(&sub) {
        Some(i) => string[..i].chars().count() as i64,
        None => -1_i64,
    }
}
