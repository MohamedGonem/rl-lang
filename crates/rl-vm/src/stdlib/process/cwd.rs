use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_cwd(_: &mut Vm) -> VmValue {
    match std::env::current_dir() {
        Ok(p) => vok!(vs!(p.to_string_lossy().to_string())),
        Err(e) => verr!(vs!(format!("cwd: {}", e))),
    }
}

pub fn std_set_cwd(_: &mut Vm, path: String) -> VmValue {
    match std::env::set_current_dir(&path) {
        Ok(_) => vok!(vnl!()),
        Err(e) => {
            verr!(vs!(format!(
                "set_cwd: failed to change to \"{}\": {}",
                path, e
            )))
        }
    }
}
