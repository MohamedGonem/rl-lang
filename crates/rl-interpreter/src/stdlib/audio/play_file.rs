use std::fs::File;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::new_sink,
        common::{extract_string, verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, path: Value) -> Value {
    let path = match extract_string(path, "play_file") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("play_file: {}", e))),
    };

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => return verr!(vs!(format!("play_file(\"{}\"): {}", path, e))),
    };

    let source = match rodio::Decoder::try_from(file) {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("play_file(\"{}\"): {}", path, e))),
    };

    let (sink, _stream) = match new_sink(eval) {
        Ok(pair) => pair,
        Err(e) => return verr!(vs!(format!("play_file: {}", e))),
    };

    sink.append(source);
    // Blocks the calling thread until playback finishes; `_stream` stays
    // alive for the duration of this call since it isn't dropped early.
    sink.sleep_until_end();
    vok!(vnl!())
}
