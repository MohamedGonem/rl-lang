use crate::{
    Vm,
    stdlib::macros::{verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_file_size(_: &mut Vm, path: String) -> VmValue {
    match std::fs::metadata(&path) {
        Err(e) => verr!(vs!(format!(
            "file_size: failed to read \"{}\": {}",
            path, e
        ))),

        Ok(metadata) => vok!(vi!(metadata.len() as i64)),
    }
}
