use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_number, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_number(handle, "sound_wait") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_wait: {}", e))),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => {
            h.sink.sleep_until_end();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_wait: unknown handle {}", id))),
    }
}
