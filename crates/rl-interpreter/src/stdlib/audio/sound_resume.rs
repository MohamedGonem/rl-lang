use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_number, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_number(handle, "sound_resume") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_resume: {}", e))),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => {
            h.sink.play();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_resume: unknown handle {}", id))),
    }
}
