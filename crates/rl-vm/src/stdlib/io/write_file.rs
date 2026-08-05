use crate::{Vm, stdlib::macros::{verr, vnl, vok, vs}, values::VmValue};

pub fn std_write_file(_: &mut Vm, file: String, content: String) -> VmValue {
    match std::fs::write(&file, content) {
        Ok(_) => vok!(vnl!()),
        Err(e) => {
            verr!(vs!(format!(
                "write_file: failed to write \"{}\": {}",
                file, e
            )))
        }
    }
}
