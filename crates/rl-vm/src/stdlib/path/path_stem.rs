use crate::{
    Vm,
    stdlib::macros::{vnl, vs},
    values::VmValue,
};

pub fn std_path_stem(_: &mut Vm, path: String) -> VmValue {
    match std::path::Path::new(&path).file_stem() {
        Some(stem) => vs!(stem.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
