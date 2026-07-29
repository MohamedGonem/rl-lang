use rl_ast::statements::HandleKind;

use crate::{
    Vm,
    stdlib::{
        common::extract_handle,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, handle: VmValue) -> VmValue {
    let id = match extract_handle(handle, HandleKind::Audio, "sound_wait") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("sound_wait: {}", e))),
    };

    match vm.audio_handles.get(&id) {
        Some(h) => {
            h.sink.sleep_until_end();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_wait: unknown handle {}", id))),
    }
}
