use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn std_rename_file(_: &mut Vm, path: String, new_name: String) -> VmValue {
    let old_path = std::path::Path::new(&path);
    let new_path = match old_path.parent() {
        Some(parent) => parent.join(&new_name),
        None => std::path::PathBuf::from(&new_name),
    };

    if let Err(e) = std::fs::rename(old_path, &new_path) {
        return verr!(vs!(format!(
            "rename_file(): failed to rename \"{}\" to \"{}\": {}",
            path,
            new_path.to_string_lossy(),
            e
        )));
    };

    vok!(vs!(new_path.to_string_lossy().to_string()))
}
