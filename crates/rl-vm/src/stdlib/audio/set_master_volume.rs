use crate::{
    Vm,
    stdlib::{
        audio::common::extract_float,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, volume: VmValue) -> VmValue {
    let volume = match extract_float(volume, "set_master_volume") {
        Ok(v) => v as f32,
        Err(e) => return verr!(vs!(format!("set_master_volume: {}", e))),
    };

    vm.audio_master_volume = volume;
    for handle in vm.audio_handles.values() {
        handle.sink.set_volume(handle.base_volume * volume);
    }
    vok!(vnl!())
}
