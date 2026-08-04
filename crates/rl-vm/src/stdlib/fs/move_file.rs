use crate::{
    Vm,
    stdlib::macros::{verr, vnl, vok, vs},
    values::VmValue,
};

pub fn std_move_file(_: &mut Vm, src: String, dst: String) -> VmValue {
    if let Err(e) = std::fs::rename(&src, &dst) {
        return verr!(vs!(format!(
            "move_file: failed to move \"{}\" to \"{}\": {}",
            src, dst, e
        )));
    };
    vok!(vnl!())
}
