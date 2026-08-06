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
    let id = match extract_handle(handle, HandleKind::Audio, "sound_resume") {
        Ok(id) => id,
        Err(e) => return verr!(vs!(format!("sound_resume: {}", e))),
    };

    match vm.audio_handles.get(&id) {
        Some(h) => {
            h.sink.play();
            vok!(vnl!())
        }
        None => verr!(vs!(format!("sound_resume: unknown handle {}", id))),
    }
}
