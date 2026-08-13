use crate::values::VmValue;
use rl_ast::statements::HandleKind;
use rl_utils::{
    errors::{Error, Reason},
    span::Span,
};

// Re-exported so mirrored stdlib files can import the value-construction
// macros from `common` or `macros`.
pub(crate) use crate::stdlib::macros::{try_fn, vb, vby, verr, vi, vnl, vok, vs};

pub fn check_arity_range(
    args: &[VmValue],
    expected_from: usize,
    expected_to: usize,
    name: &str,
    span: Span,
) -> Result<(), Error> {
    let len = args.len();
    if !(expected_from..=expected_to).contains(&len) {
        let message = format!(
            "{}: expected from {} to {} arg(s), got {}",
            name,
            expected_from,
            expected_to,
            args.len()
        );

        return Err(Error::at(Reason::Runtime, message, span));
    }
    Ok(())
}

pub fn check_type(value: &VmValue, expected: &str, name: &str) -> Result<(), String> {
    match (value, expected) {
        (VmValue::Bool(_), "bool")
        | (VmValue::Str(_), "string")
        | (VmValue::Int(_), "int")
        | (VmValue::Byte(_), "byte")
        | (VmValue::Char(_), "char")
        | (VmValue::Error(_), "error")
        | (VmValue::Float(_), "float")
        | (VmValue::Function { .. }, "fn")
        | (VmValue::Null, "none" | "null")
        | (VmValue::Ok(_), "ok")
        | (VmValue::Tuple(_), "tuple" | "()")
        | (VmValue::Arr(_), "arr") => Ok(()),

        (other_val, other_exp) => Err(format!(
            "{}: expected {} type, got {}",
            name,
            other_exp,
            other_val.type_name()
        )),
    }
}

pub fn extract_string(value: VmValue, name: &str) -> Result<String, String> {
    check_type(&value, "string", name)?;
    let VmValue::Str(v) = value else {
        unreachable!()
    };
    Ok(v.to_string())
}

pub fn extract_int(value: VmValue, name: &str) -> Result<i64, String> {
    check_type(&value, "int", name)?;
    let VmValue::Int(v) = value else {
        unreachable!()
    };
    Ok(v)
}

pub fn extract_number(value: VmValue, name: &str) -> Result<u64, String> {
    if check_type(&value, "int", name).is_err() && check_type(&value, "byte", name).is_err() {
        return Err(format!(
            "{}: expected int or byte type, got {}",
            name,
            value.type_name()
        ));
    }

    match value {
        VmValue::Int(i) => Ok(i as u64),
        VmValue::Byte(b) => Ok(b as u64),
        _ => {
            unreachable!()
        }
    }
}

/// Unwraps a `VmValue::Handle` of the expected kind into its raw id.
/// This is the runtime half of the checker's static rejection - it catches
/// a wrong-module handle that reached here anyway (e.g. through an `array[?]`
/// or anything else the static checker can't fully see through).
pub fn extract_handle(value: VmValue, expected: HandleKind, name: &str) -> Result<u64, String> {
    match value {
        VmValue::Handle { kind, id } if kind == expected => Ok(id),
        VmValue::Handle { kind, .. } => Err(format!(
            "{}: expected a {:?} handle, got a {:?} handle",
            name, expected, kind
        )),
        other => Err(format!(
            "{}: expected a handle, got {}",
            name,
            other.type_name()
        )),
    }
}
