use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::extract_float,
        common::{extract_number, verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, speed: Value) -> Value {
    let id = match extract_number(handle, "sound_set_speed") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_set_speed: {}", e))),
    };
    let speed = match extract_float(speed, "sound_set_speed") {
        Ok(s) => s as f32,
        Err(e) => return verr!(vs!(format!("sound_set_speed: {}", e))),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => {
            h.sink.set_speed(speed);
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_set_speed: unknown handle {}", id))),
    }
}
