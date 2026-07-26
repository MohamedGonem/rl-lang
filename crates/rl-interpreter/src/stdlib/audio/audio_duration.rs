use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::probe_file,
        common::{extract_string, verr, vi, vok, vs},
    },
    values::Value,
};

pub fn func(_: &mut Evaluator, path: Value) -> Value {
    let path = match extract_string(path, "audio_duration") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("audio_duration: {}", e))),
    };

    match probe_file(&path) {
        Ok(info) => vok!(vi!(info.duration_ms)),
        Err(e) => verr!(vs!(format!("audio_duration(\"{}\"): {}", path, e))),
    }
}
