use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_number, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_number(handle, "sound_stop") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_stop: {}", e))),
    };

    match eval.audio_handles.remove(&id) {
        Some(h) => {
            h.sink.stop();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_stop: unknown handle {}", id))),
    }
}
