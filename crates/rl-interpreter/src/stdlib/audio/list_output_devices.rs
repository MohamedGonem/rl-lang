use cpal::traits::{DeviceTrait, HostTrait};
use rl_ast::statements::TypeAnnotation;

use crate::{
    evaluator::Evaluator,
    stdlib::common::{verr, vok, vs},
    values::Value,
};

pub fn func(_: &mut Evaluator) -> Value {
    let host = cpal::default_host();
    let devices = match host.output_devices() {
        Ok(d) => d,
        Err(e) => return verr!(vs!(format!("list_output_devices: {}", e))),
    };

    let items: Vec<Value> = devices
        .filter_map(|d| d.description().ok())
        .map(|desc| vs!(desc.name().to_string()))
        .collect();

    vok!(Value::Values {
        items_type: TypeAnnotation::String,
        items,
    })
}
