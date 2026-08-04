use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_rmdir_all(_: &mut Vm, path: String) -> VmValue {
    if let Err(e) = std::fs::remove_dir_all(&path) {
        return verr!(vs!(format!(
            "rmdir_all: failed to delete \"{}\": {}",
            path, e
        )));
    };
    vok!(vnl!())
}
