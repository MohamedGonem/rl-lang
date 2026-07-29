use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        macros::{vb, verr, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_is_finished") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("sound_is_finished: {}", e))),
    };

    match vm.audio_handles.get(&id) {
        Some(h) => vok!(vb!(h.sink.empty())),
        None => verr!(vs!(format!("sound_is_finished: unknown handle {}", id))),
    }
}
