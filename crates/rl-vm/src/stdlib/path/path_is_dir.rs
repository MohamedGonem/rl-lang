use crate::Vm;

pub fn std_path_is_dir(_: &mut Vm, path: String) -> bool {
    std::path::Path::new(&path).is_dir()
}
