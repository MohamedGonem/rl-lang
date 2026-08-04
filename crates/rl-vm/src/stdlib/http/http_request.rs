use crate::{
    stdlib::{
        common::{check_arity_range, extract_string, verr, vs},
        http::common::ureq_result_to_value,
    },
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use rl_utils::span::Span;

pub fn func(_: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    check_arity_range(&args, 2, 4, "http_request", Span::dummy())?;

    let method = match extract_string(args[0].clone(), "http_request") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("http_request: {e}")))),
    };
    let url = match extract_string(args[1].clone(), "http_request") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("http_request: {e}")))),
    };
    let body = match args.get(2) {
        Some(VmValue::Str(s)) => Some(s.to_string()),
        Some(other) => {
            return Ok(verr!(vs!(format!(
                "http_request: expects a string body, got {}",
                other.type_name()
            ))));
        }
        None => None,
    };

    let mut request = ureq::request(&method, &url);
    if let Some(VmValue::Arr(items)) = args.get(3) {
        for item in items.iter() {
            match item {
                VmValue::Tuple(pair) if pair.len() == 2 => {
                    if let (VmValue::Str(name), VmValue::Str(value)) = (&pair[0], &pair[1]) {
                        request = request.set(name, value);
                        continue;
                    }
                    return Ok(verr!(vs!(
                        "http_request: expects headers as an array of (string, string) tuples"
                            .to_string()
                    )));
                }
                _ => {
                    return Ok(verr!(vs!(
                        "http_request: expects headers as an array of (string, string) tuples"
                            .to_string()
                    )));
                }
            }
        }
    } else if args.get(3).is_some() {
        return Ok(verr!(vs!(
            "http_request: expects headers as an array of (string, string) tuples".to_string()
        )));
    }

    let result = match &body {
        Some(b) => request.send_string(b),
        None => request.call(),
    };
    Ok(ureq_result_to_value(&url, result))
}
