use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_rmdir(_: &mut Vm, path: String) -> VmValue {
    if let Err(e) = std::fs::remove_dir(&path) {
        return verr!(vs!(format!("rmdir: failed to delete \"{}\": {}", path, e)));
    };
    vok!(vnl!())
}
