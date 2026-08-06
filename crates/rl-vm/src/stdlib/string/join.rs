use crate::{
    stdlib::macros::{verr, vok, vs},
    values::VmValue,
    vm_logic::Vm,
};

pub fn std_join(_: &mut Vm, strings_array: VmValue, delim: String) -> VmValue {
    match strings_array {
        VmValue::Arr(array) => {
            let mut strings: Vec<String> = vec![];
            for v in array.iter() {
                match v {
                    VmValue::Int(i) => strings.push(format!("{}", i)),
                    VmValue::Float(f) => strings.push(format!("{}", f)),
                    VmValue::Bool(b) => strings.push(format!("{}", b)),
                    VmValue::Str(s) => strings.push(s.to_string()),
                    VmValue::Char(c) => strings.push(c.to_string()),
                    VmValue::Null => strings.push("null".to_string()),
                    VmValue::Function { .. } | VmValue::Native(_) | VmValue::Closure { .. } => {
                        return verr!(vs!(
                            "functions/lambdas/enclosures are not supported via join()".to_string()
                        ));
                    }
                    _ => {}
                }
            }
            vok!(vs!(strings.join(&delim)))
        }
        _ => verr!(vs!("join() expects an array as first argument".to_string())),
    }
}
