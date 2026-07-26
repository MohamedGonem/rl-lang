use crate::{
    Vm,
    stdlib::{common::extract_number, macros::{verr, vf, vok, vs}},
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_number(handle, "sound_get_volume") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_get_volume: {}", e))),
    };

    match vm.audio_handles.get(&id) {
        Some(h) => vok!(vf!(h.base_volume as f64)),
        None => verr!(vs!(format!("sound_get_volume: unknown handle {}", id))),
    }
}
