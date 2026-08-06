use crate::{
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn func(eval: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    if args.len() > 1 {
        return Err(eval.err(format!(
            "unreachable: expects 0 or 1 arguments, got {}",
            args.len()
        )));
    }

    let message = match args.into_iter().next() {
        Some(VmValue::Str(s)) => format!("internal error: entered unreachable code: {}", s),
        Some(other) => {
            return Err(eval.err(format!(
                "unreachable: expects a string message, got {}",
                other.type_name()
            )));
        }
        None => "internal error: entered unreachable code".to_string(),
    };

    Err(eval.err(message))
}
