use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::extract_float,
        common::{extract_number, verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, handle: Value, volume: Value) -> Value {
    let id = match extract_number(handle, "sound_set_volume") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_set_volume: {}", e))),
    };
    let volume = match extract_float(volume, "sound_set_volume") {
        Ok(v) => v as f32,
        Err(e) => return verr!(vs!(format!("sound_set_volume: {}", e))),
    };

    let master = eval.audio_master_volume;
    match eval.audio_handles.get_mut(&id) {
        Some(h) => {
            h.base_volume = volume;
            h.sink.set_volume(volume * master);
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_set_volume: unknown handle {}", id))),
    }
}
