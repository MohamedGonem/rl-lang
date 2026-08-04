use crate::{
    Vm,
    stdlib::macros::{vnl, vs},
    values::VmValue,
};

pub fn std_path_extension(_: &mut Vm, path: String) -> VmValue {
    match std::path::Path::new(&path).extension() {
        Some(ext) => vs!(ext.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
