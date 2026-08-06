use crate::{
    evaluator::Evaluator,
    stdlib::{
        c::common::cache_dir,
        common::{verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(_eval: &mut Evaluator) -> Value {
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
