use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_handle, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_stop") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.audio_handles.remove(&id) {
        Some(h) => {
            h.sink.stop();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_stop: unknown handle {}", id))),
    }
}
