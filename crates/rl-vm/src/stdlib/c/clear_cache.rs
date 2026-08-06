use crate::{
    Vm,
    stdlib::{
        c::common::cache_dir,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn std_clear_cache(_vm: &mut Vm) -> VmValue {
    let dir = cache_dir();
    match std::fs::remove_dir_all(&dir) {
        Ok(()) => vok!(vnl!()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => vok!(vnl!()),
        Err(e) => verr!(vs!(format!(
            "clear_cache: failed to remove {}: {}",
            dir.display(),
            e
        ))),
    }
}
