use crate::{
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn func(eval: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    if args.len() > 1 {
        return Err(eval.err(format!(
            "panic: expects 0 or 1 arguments, got {}",
            args.len()
        )));
    }
    let message = match args.into_iter().next() {
        Some(VmValue::Str(s)) => s.to_string(),
        Some(other) => {
            return Err(eval.err(format!(
                "panic: expects a string message, got {}",
                other.type_name()
            )));
        }
        None => "explicit panic".to_string(),
    };
    Err(eval.err(message))
}
