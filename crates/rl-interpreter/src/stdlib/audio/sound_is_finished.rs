use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_number, vb, verr, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_number(handle, "sound_is_finished") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_is_finished: {}", e))),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => vok!(vb!(h.sink.empty())),
        None => verr!(vs!(format!("sound_is_finished: unknown handle {}", id))),
    }
}
