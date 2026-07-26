use crate::{
    Vm,
    stdlib::{common::extract_number, macros::{verr, vnl, vok, vs}},
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_number(handle, "sound_stop") {
        Ok(n) => n as i64,
        Err(e) => return verr!(vs!(format!("sound_stop: {}", e))),
    };

    match vm.audio_handles.remove(&id) {
        Some(h) => {
            h.sink.stop();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_stop: unknown handle {}", id))),
    }
}
