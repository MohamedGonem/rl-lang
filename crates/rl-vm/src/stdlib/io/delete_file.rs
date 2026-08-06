use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_delete_file(_: &mut Vm, file: String) -> VmValue {
    match std::fs::remove_file(&file) {
        Ok(_) => vok!(vnl!()),
        Err(e) => verr!(vs!(format!(
            "delete_file: failed to read \"{}\": {}",
            file, e
        ))),
    }
}
