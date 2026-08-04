use crate::Vm;

pub fn std_path_join(_: &mut Vm, path: String, target: String) -> String {
    std::path::PathBuf::from(&path)
        .join(&target)
        .to_string_lossy()
        .to_string()
}
