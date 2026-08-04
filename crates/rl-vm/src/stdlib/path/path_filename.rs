use crate::{
    Vm,
    stdlib::macros::{vnl, vs},
    values::VmValue,
};

pub fn std_path_filename(_: &mut Vm, path: String) -> VmValue {
    match std::path::Path::new(&path).file_name() {
        Some(name) => vs!(name.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
