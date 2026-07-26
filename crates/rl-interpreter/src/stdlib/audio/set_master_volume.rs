use crate::{
    evaluator::Evaluator,
    stdlib::{
        audio::common::extract_float,
        common::{verr, vnl, vok, vs},
    },
    values::Value,
};

pub fn func(eval: &mut Evaluator, volume: Value) -> Value {
    let volume = match extract_float(volume, "set_master_volume") {
        Ok(v) => v as f32,
        Err(e) => return verr!(vs!(format!("set_master_volume: {}", e))),
    };

    eval.audio_master_volume = volume;
    // Rescale every currently playing sound, not just future ones.
    for handle in eval.audio_handles.values() {
        handle.sink.set_volume(handle.base_volume * volume);
    }
    vok!(vnl!())
}
