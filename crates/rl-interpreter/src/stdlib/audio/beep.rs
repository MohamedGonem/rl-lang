use std::time::Duration;

use rodio::source::{SineWave, Source};

use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::{extract_float, new_sink},
        common::{extract_number, verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, freq: Value, duration_ms: Value) -> Value {
    let freq = match extract_float(freq, "beep") {
        Ok(f) => f,
        Err(e) => return verr!(vs!(format!("beep: {}", e))),
    };
    let duration_ms = match extract_number(duration_ms, "beep") {
        Ok(n) => n,
        Err(e) => return verr!(vs!(format!("beep: {}", e))),
    };

    let (sink, _stream) = match new_sink(eval) {
        Ok(pair) => pair,
        Err(e) => return verr!(vs!(format!("beep: {}", e))),
    };

    let source = SineWave::new(freq as f32).take_duration(Duration::from_millis(duration_ms));
    sink.append(source);
    sink.sleep_until_end();
    vok!(vnl!())
}
