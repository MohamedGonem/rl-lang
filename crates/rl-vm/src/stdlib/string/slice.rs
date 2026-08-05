use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_slice(_: &mut Vm, string: String, start: i64, end: i64) -> VmValue {
    let chars = string.chars();
    let chars_count = chars.clone().count();
    if start as usize >= chars_count || end as usize > chars_count {
        return verr!(vs!(format!(
            "index out of bounds string legth: {}, found start: {} and end: {}",
            chars_count, start, end
        )));
    }

    vok!(vs!(chars
        .skip(start as usize)
        .take(end as usize - start as usize)
        .collect::<String>()))
}
