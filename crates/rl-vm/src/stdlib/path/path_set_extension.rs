use crate::Vm;

pub fn std_path_set_extension(_: &mut Vm, path: String, target: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.set_extension(&target);
    buf.to_string_lossy().to_string()
}
