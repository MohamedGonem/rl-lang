use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_mkdir_all(_: &mut Vm, path: String) -> VmValue {
    if let Err(e) = std::fs::create_dir_all(&path) {
        return verr!(vs!(format!(
            "mkdir_all: failed to create \"{}\": {}",
            path, e
        )));
    };
    vok!(vnl!())
}
