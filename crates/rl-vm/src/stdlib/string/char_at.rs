use crate::{
    Vm,
    stdlib::macros::{vc, verr, vok, vs},
    values::VmValue,
};

pub fn std_char_at(_: &mut Vm, string: String, index: i64) -> VmValue {
    if index < 0 {
        return verr!(vs!(format!("index cannot be negative: {}", index)));
    }
    let mut chars = string.chars();
    let chars_count = chars.clone().count();
    if index as usize >= chars_count {
        verr!(vs!(format!(
            "index out of bounds string length is {} , used {}",
            chars_count, index
        )))
    } else {
        vok!(vc!(chars.nth(index as usize).unwrap()))
    }
}
