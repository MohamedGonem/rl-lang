use crate::{Vm, stdlib::macros::{verr, vnl, vok, vs}, values::VmValue};
use std::{fs::OpenOptions, io::Write};

pub fn std_append_file(_: &mut Vm, file: String, content: String) -> VmValue {
    let mut file_data = match OpenOptions::new().append(true).create(true).open(&file) {
        Ok(fd) => fd,
        Err(e) => {
            return verr!(vs!(format!(
                "append_file: failed to open \"{}\": {}",
                file, e
            )));
        }
    };

    match file_data.write_all(content.as_bytes()) {
        Err(e) => verr!(vs!(format!(
            "append_file: failed to append \"{}\": {}",
            file, e
        ))),

        Ok(_) => vok!(vnl!()),
    }
}
