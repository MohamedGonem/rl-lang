use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_read_lines(_: &mut Vm, file: String) -> VmValue {
    let data = match std::fs::read_to_string(&file) {
        Ok(d) => d,
        Err(e) => {
            return verr!(vs!(format!(
                "read_lines: failed to read \"{}\": {}",
                file, e
            )));
        }
    };
    vok!(VmValue::Arr(Rc::new(
        data.lines().map(|line| vs!(String::from(line))).collect(),
    )))
}
