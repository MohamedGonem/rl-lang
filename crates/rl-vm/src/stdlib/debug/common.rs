use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn as_f64(value: &VmValue) -> Option<f64> {
    match value {
        VmValue::Int(i) => Some(*i as f64),
        VmValue::Float(f) => Some(*f),
        VmValue::Byte(b) => Some(*b as f64),
        _ => None,
    }
}

pub fn assert_eq_message(
    a: &VmValue,
    b: &VmValue,
    custom: Option<&VmValue>,
    name: &str,
    expected_equal: bool,
) -> VmValue {
    let op = if expected_equal { "!=" } else { "==" };
    let default_msg = format!(
        "{} failed: left `{}` ({}) {} right `{}` ({})",
        name,
        a,
        a.type_name(),
        op,
        b,
        b.type_name()
    );

    match custom {
        Some(VmValue::Str(s)) => vok!(vs!(format!("{}: {}", s, default_msg))),
        Some(other) => verr!(vs!(format!(
            "{}() expects a string message, got {}",
            name,
            other.type_name()
        ))),
        None => vok!(vs!(default_msg)),
    }
}

pub fn assert_cmp(
    eval: &mut Vm,
    args: Vec<VmValue>,
    name: &str,
    op: fn(f64, f64) -> bool,
) -> Result<VmValue, VmError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(eval.err(format!(
            "{}() expects 2 or 3 arguments, got {}",
            name,
            args.len()
        )));
    }

    let (a, b) = (&args[0], &args[1]);
    let (fa, fb) = match (as_f64(a), as_f64(b)) {
        (Some(fa), Some(fb)) => (fa, fb),
        _ => {
            return Err(eval.err(format!(
                "{}: expects numeric arguments, got {} and {}",
                name,
                a.type_name(),
                b.type_name()
            )));
        }
    };

    if !op(fa, fb) {
        let default_msg = format!("{} failed: `{}` vs `{}`", name, a, b);
        let message = match args.get(2) {
            Some(VmValue::Str(s)) => format!("{}: {}", s, default_msg),
            Some(other) => {
                return Err(eval.err(format!(
                    "{}: expects a string message, got {}",
                    name,
                    other.type_name()
                )));
            }
            None => default_msg,
        };
        return Err(eval.err(message));
    }

    Ok(VmValue::Null)
}
