use crate::{Vm, stdlib::macros::{verr, vok, vs}, values::VmValue};

pub fn std_read_file(_: &mut Vm, file: String) -> VmValue {
    let data = match std::fs::read_to_string(&file) {
        Ok(d) => d,
        Err(e) => {
            return verr!(vs!(format!(
                "read_file: failed to read \"{}\": {}",
                file, e
            )));
        }
    };
    vok!(vs!(data))
}
