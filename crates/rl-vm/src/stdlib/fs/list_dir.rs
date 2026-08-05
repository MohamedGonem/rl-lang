use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};
use std::rc::Rc;

pub fn std_list_dir(_: &mut Vm, path: String) -> VmValue {
    match std::fs::read_dir(&path) {
        Err(e) => {
            verr!(vs!(format!("list_dir: failed to read \"{}\": {}", path, e)))
        }
        Ok(d) => {
            vok!(VmValue::Arr(Rc::new(
                d.filter_map(|i| i.ok())
                    .map(|i| i.path().to_string_lossy().to_string())
                    .map(|s| VmValue::Str(Rc::from(s)))
                    .collect::<Vec<VmValue>>()
            )))
        }
    }
}
