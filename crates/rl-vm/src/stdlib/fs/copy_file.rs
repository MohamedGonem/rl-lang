use crate::{
    Vm,
    stdlib::macros::{verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_copy_file(_: &mut Vm, src: String, dst: String) -> VmValue {
    let bytes = match std::fs::copy(&src, &dst) {
        Ok(b) => b,
        Err(e) => {
            return verr!(vs!(format!(
                "copy_file: failed to copy \"{}\" to \"{}\": {}",
                src, dst, e
            )));
        }
    };
    vok!(vi!(bytes as i64))
}
