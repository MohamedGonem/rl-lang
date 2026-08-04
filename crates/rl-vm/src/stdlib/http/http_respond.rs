use crate::{
    stdlib::{
        common::{check_arity_range, extract_handle, extract_string, verr, vok, vs},
        http::HttpHandle,
    },
    values::VmValue,
    vm_logic::{Vm, VmError},
};
use rl_ast::statements::HandleKind;
use rl_utils::span::Span;

pub fn func(eval: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    check_arity_range(&args, 3, 4, "http_post", Span::dummy())?;

    let id = match extract_handle(args[0].clone(), HandleKind::Http, "http_respond") {
        Ok(id) => id,
        Err(e) => return Ok(verr!(vs!(e))),
    };

    let status = match &args[1] {
        VmValue::Int(n) if (100..=599).contains(n) => *n as u16,
        other => {
            return Ok(verr!(vs!(format!(
                "http_respond: expects a valid HTTP status int, got {}",
                other.type_name()
            ))));
        }
    };

    let body = match extract_string(args[2].clone(), "http_respond") {
        Ok(s) => s,
        Err(e) => return Ok(verr!(vs!(format!("{e}")))),
    };
    let content_type = match args.get(3) {
        Some(VmValue::Str(s)) => Some(s.to_string()),
        Some(other) => {
            return Ok(verr!(vs!(format!(
                "http_respond: expects a string content_type, got {}",
                other.type_name()
            ))));
        }
        None => None,
    };

    let request = match eval.http_handles.remove(&id) {
        Some(HttpHandle::Request(request)) => request,
        Some(other) => {
            eval.http_handles.insert(id, other);
            return Ok(verr!(vs!(format!(
                "http_respond: handle {} is not a request",
                id
            ))));
        }
        None => return Ok(verr!(vs!(format!("http_respond: unknown handle {}", id)))),
    };

    let mut response = tiny_http::Response::from_string(body).with_status_code(status);
    if let Some(ct) = content_type
        && let Ok(header) = tiny_http::Header::from_bytes(&b"Content-Type"[..], ct.as_bytes())
    {
        response = response.with_header(header);
    }

    match request.respond(response) {
        Ok(()) => Ok(vok!(VmValue::Null)),
        Err(e) => Ok(verr!(vs!(format!("http_respond: {}", e)))),
    }
}
