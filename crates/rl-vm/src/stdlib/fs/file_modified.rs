use crate::{
    Vm,
    stdlib::macros::{verr, vi, vok, vs},
    values::VmValue,
};

pub fn std_file_modified(_: &mut Vm, path: String) -> VmValue {
    match std::fs::metadata(&path) {
        Err(e) => verr!(vs!(format!(
            "file_modified: failed to read \"{}\": {}",
            path, e
        ))),

        Ok(metadata) => match metadata.modified() {
            Err(e) => verr!(vs!(format!(
                "file_modified: could not get modification time for \"{}\": {}",
                path, e
            ))),
            Ok(modified) => match modified.duration_since(std::time::UNIX_EPOCH) {
                Err(e) => {
                    verr!(vs!(format!(
                        "file_modified: modification time before epoch for \"{}\": {}",
                        path, e
                    )))
                }
                Ok(t) => {
                    vok!(vi!(t.as_secs() as i64))
                }
            },
        },
    }
}
