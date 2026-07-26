use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::probe_file,
        common::{extract_string, verr, vi, vok, vs},
    },
    values::Value,
};

pub fn func(_: &mut Evaluator, path: Value) -> Value {
    let path = match extract_string(path, "audio_file_info") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("audio_file_info: {}", e))),
    };

    match probe_file(&path) {
        Ok(info) => vok!(Value::Tuple(vec![
            vi!(info.channels as i64),
            vi!(info.sample_rate as i64),
            vi!(info.duration_ms),
            vs!(info.format_name),
        ])),
        Err(e) => verr!(vs!(format!("audio_file_info(\"{}\"): {}", path, e))),
    }
}
