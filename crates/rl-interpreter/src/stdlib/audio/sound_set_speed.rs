use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::extract_float,
        common::{extract_handle, verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, speed: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_set_speed") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
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
