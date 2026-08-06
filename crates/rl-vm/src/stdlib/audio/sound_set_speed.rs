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

pub fn func(vm: &mut Vm, handle: VmValue, speed: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_set_speed") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("sound_set_speed: {}", e))),
    };
    let speed = match extract_float(speed, "sound_set_speed") {
        Ok(s) => s as f32,
        Err(e) => return verr!(vs!(format!("sound_set_speed: {}", e))),
    };

    match vm.audio_handles.get(&id) {
        Some(h) => {
            h.sink.set_speed(speed);
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_set_speed: unknown handle {}", id))),
    }
}
