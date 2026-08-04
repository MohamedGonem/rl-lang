use crate::{
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn func(eval: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    if args.is_empty() || args.len() > 2 {
        return Err(eval.err(format!(
            "assert: expects 1 or 2 arguments, got {}",
            args.len()
        )));
    }

    let cond = match &args[0] {
        VmValue::Bool(b) => *b,
        other => {
            return Err(eval.err(format!(
                "assert: expects a bool condition, got {}",
                other.type_name()
            )));
        }
    };

    if !cond {
        let message = match args.get(1) {
            Some(VmValue::Str(s)) => s.to_string(),
            Some(other) => {
                return Err(eval.err(format!(
                    "assert: expects a string message, got {}",
                    other.type_name()
                )));
            }
            None => "assertion failed".to_string(),
        };
        return Err(eval.err(message));
    }

    Ok(VmValue::Null)
}
