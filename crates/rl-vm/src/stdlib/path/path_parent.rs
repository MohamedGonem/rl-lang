use crate::{
    Vm,
    stdlib::macros::{vnl, vs},
    values::VmValue,
};

pub fn std_path_parent(_: &mut Vm, path: String) -> VmValue {
    match std::path::Path::new(&path).parent() {
        Some(p) => vs!(p.to_string_lossy().to_string()),
        None => vnl!(),
    }
}
