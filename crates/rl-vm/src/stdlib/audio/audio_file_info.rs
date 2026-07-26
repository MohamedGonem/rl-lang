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
    let path = match extract_string(path, "audio_file_info") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("audio_file_info: {}", e))),
    };

    match probe_file(&path) {
        Ok(info) => vok!(VmValue::Tuple(std::rc::Rc::new(vec![
            vi!(info.channels as i64),
            vi!(info.sample_rate as i64),
            vi!(info.duration_ms),
            vs!(info.format_name),
        ]))),
        Err(e) => verr!(vs!(format!("audio_file_info(\"{}\"): {}", path, e))),
    }
}
