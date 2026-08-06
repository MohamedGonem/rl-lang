use crate::{
    stdlib::common::extract_string,
    stdlib::debug::common::assert_eq_message,
    values::VmValue,
    vm_logic::{Vm, VmError},
};

pub fn func(eval: &mut Vm, args: Vec<VmValue>) -> Result<VmValue, VmError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(eval.err(format!(
            "assert_eq: expects 2 or 3 arguments, got {}",
            args.len()
        )));
    }

    let (a, b) = (&args[0], &args[1]);
    if a != b {
        let err = match assert_eq_message(a, b, args.get(2), "assert_eq", true) {
            VmValue::Ok(k) => *k,
            VmValue::Err(e) => *e,
            _ => {
                unreachable!()
            }
        };
        let err_string = match extract_string(err, "assert_eq") {
            Err(a) | Ok(a) => a,
        };

        return Err(eval.err(err_string));
    }
    Ok(VmValue::Null)
}
