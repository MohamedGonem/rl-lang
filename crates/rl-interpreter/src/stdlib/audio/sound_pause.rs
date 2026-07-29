use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_handle, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_pause") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => {
            h.sink.pause();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_pause: unknown handle {}", id))),
    }
}
