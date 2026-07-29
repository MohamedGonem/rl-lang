use rl_ast::statements::HandleKind;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_handle, verr, vf, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value) -> Value {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_get_volume") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(e)),
    };

    // Returns the volume the user asked for via `sound_set_volume`, not the
    // master-volume-scaled value actually applied to the sink.
    match eval.audio_handles.get(&id) {
        Some(h) => vok!(vf!(h.base_volume as f64)),
        None => verr!(vs!(format!("sound_get_volume: unknown handle {}", id))),
    }
}
