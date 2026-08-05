use crate::{Vm, stdlib::http::common::ureq_result_to_value, values::VmValue};

pub fn func(_: &mut Vm, url: String) -> VmValue {
    let result = ureq::get(&url).call();
    ureq_result_to_value(&url, result)
}
