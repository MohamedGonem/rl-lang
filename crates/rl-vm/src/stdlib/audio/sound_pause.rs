use crate::{
    Vm,
    stdlib::{common::extract_number, macros::{verr, vnl, vok, vs}},
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_number(handle, "sound_pause") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_pause: {}", e))),
    };

    match vm.audio_handles.get(&id) {
        Some(h) => {
            h.sink.pause();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_pause: unknown handle {}", id))),
    }
}
