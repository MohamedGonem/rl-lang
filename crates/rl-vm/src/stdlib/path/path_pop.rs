use crate::Vm;

pub fn std_path_pop(_: &mut Vm, path: String) -> String {
    let mut buf = std::path::PathBuf::from(&path);
    buf.pop();
    buf.to_string_lossy().to_string()
}
