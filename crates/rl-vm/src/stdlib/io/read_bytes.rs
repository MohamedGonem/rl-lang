use crate::{
    Vm,
    stdlib::macros::{vby, verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_read_bytes(_: &mut Vm, file: String) -> VmValue {
    let data = match std::fs::read(&file) {
        Err(e) => {
            return verr!(vs!(format!(
                "read_bytes: failed to read \"{}\": {}",
                file, e
            )));
        }
        Ok(d) => d.into_iter().map(|b| vby!(b)).collect::<Vec<VmValue>>(),
    };
    vok!(VmValue::Arr(Rc::new(data)))
}
