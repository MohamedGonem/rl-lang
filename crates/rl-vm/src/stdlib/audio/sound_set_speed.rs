use crate::{
    Vm,
    stdlib::{
        audio::common::extract_float,
        common::extract_number,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue, speed: VmValue) -> VmValue {
    let id = match extract_number(handle, "sound_set_speed") {
        Ok(n) => n as i64,
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
