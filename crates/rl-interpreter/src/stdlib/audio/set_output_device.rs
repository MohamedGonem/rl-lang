use cpal::traits::{DeviceTrait, HostTrait};

use crate::{
    evaluator::Evaluator,
    stdlib::common::{extract_string, verr, vnl, vok, vs},
    values::Value,
};

pub fn func(eval: &mut Evaluator, name: Value) -> Value {
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

    eval.audio_output_device = Some(name);
    vok!(vnl!())
}
