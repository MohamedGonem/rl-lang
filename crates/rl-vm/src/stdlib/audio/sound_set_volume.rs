use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        audio::common::extract_float,
        common::extract_handle,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue, volume: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_set_volume") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("sound_set_volume: {}", e))),
    };
    let volume = match extract_float(volume, "sound_set_volume") {
        Ok(v) => v as f32,
        Err(e) => return verr!(vs!(format!("sound_set_volume: {}", e))),
    };

    let master = vm.audio_master_volume;
    match vm.audio_handles.get_mut(&id) {
        Some(h) => {
            h.base_volume = volume;
            h.sink.set_volume(volume * master);
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_set_volume: unknown handle {}", id))),
    }
}
