use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_handle, vb, verr, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_is_finished") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    match eval.audio_handles.get(&id) {
        Some(h) => vok!(vb!(h.sink.empty())),
        None => verr!(vs!(format!("sound_is_finished: unknown handle {}", id))),
    }
}
