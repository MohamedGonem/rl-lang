use crate::Vm;

pub fn std_path_exists(_: &mut Vm, path: String) -> bool {
    std::path::Path::new(&path).exists()
}
