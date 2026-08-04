use crate::Vm;

pub fn std_path_is_file(_: &mut Vm, path: String) -> bool {
    std::path::Path::new(&path).is_file()
}
