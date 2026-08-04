use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_mkdir(_: &mut Vm, path: String) -> VmValue {
    if let Err(e) = std::fs::create_dir(&path) {
        return verr!(vs!(format!("mkdir: failed to create \"{}\": {}", path, e)));
    };
    vok!(vnl!())
}
