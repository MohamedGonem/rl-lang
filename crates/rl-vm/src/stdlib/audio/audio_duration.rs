use crate::{
    Vm,
    stdlib::{
        audio::common::probe_file,
        common::extract_string,
        macros::{verr, vi, vok, vs},
    },
    values::VmValue,
};

pub fn func(_: &mut Vm, path: VmValue) -> VmValue {
    let path = match extract_string(path, "audio_duration") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("audio_duration: {}", e))),
    };

    match probe_file(&path) {
        Ok(info) => vok!(vi!(info.duration_ms)),
        Err(e) => verr!(vs!(format!("audio_duration(\"{}\"): {}", path, e))),
    }
}
