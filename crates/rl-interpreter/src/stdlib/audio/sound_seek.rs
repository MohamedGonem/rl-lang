use std::time::Duration;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_number, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, position_ms: Value) -> Value {
    let id = match extract_number(handle, "sound_seek") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_seek: {}", e))),
    };
    let position_ms = match extract_number(position_ms, "sound_seek") {
        Ok(n) => n,
        Err(e) => return verr!(vs!(format!("sound_seek: {}", e))),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => match h.sink.try_seek(Duration::from_millis(position_ms)) {
            Ok(()) => vok!(vnl!()),
            Err(e) => verr!(vs!(format!("sound_seek: {}", e))),
        },
        None => verr!(vs!(format!("sound_seek: unknown handle {}", id))),
    }
}
