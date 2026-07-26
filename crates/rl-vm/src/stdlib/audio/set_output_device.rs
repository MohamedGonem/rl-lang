use cpal::traits::{DeviceTrait, HostTrait};

use crate::{
    Vm,
    stdlib::{
        common::extract_string,
        macros::{verr, vnl, vok, vs},
    },
    values::VmValue,
};

pub fn func(vm: &mut Vm, name: VmValue) -> VmValue {
    let name = match extract_string(name, "set_output_device") {
        Ok(s) => s,
        Err(e) => return verr!(vs!(format!("set_output_device: {}", e))),
    };

    let host = cpal::default_host();
    let devices = match host.output_devices() {
        Ok(d) => d,
        Err(e) => return verr!(vs!(format!("set_output_device: {}", e))),
    };

    let found = devices
        .filter_map(|d| d.description().ok())
        .any(|desc| desc.name() == name);

    if !found {
        return verr!(vs!(format!(
            "set_output_device: output device \"{}\" not found",
            name
        )));
    }

    vm.audio_output_device = Some(name);
    vok!(vnl!())
}
