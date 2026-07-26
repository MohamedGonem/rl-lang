use cpal::traits::{DeviceTrait, HostTrait};

use crate::{
    Vm,
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
};

pub fn func(_: &mut Vm) -> VmValue {
    let host = cpal::default_host();
    let devices = match host.output_devices() {
        Ok(d) => d,
        Err(e) => return verr!(vs!(format!("list_output_devices: {}", e))),
    };

    let items: Vec<VmValue> = devices
        .filter_map(|d| d.description().ok())
        .map(|desc| vs!(desc.name().to_string()))
        .collect();

    vok!(VmValue::Arr(std::rc::Rc::new(items)))
}
